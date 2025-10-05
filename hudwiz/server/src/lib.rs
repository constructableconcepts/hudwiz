pub mod auth;
pub mod config;
pub mod sso;
pub mod webtransport;

use crate::{
    auth::{AuthProvider, Credentials, DbPrimaryProvider, UserInfo, UserStore, User},
    sso::{GoogleSsoProvider, MicrosoftSsoProvider, SsoProvider},
};
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Json as JsonResponse},
    routing::{get, post, get_service},
    Extension, Json, Router,
};
use futures::sink::SinkExt;
use std::{collections::HashMap, sync::Arc};
use tower_cookies::{CookieManagerLayer, Key};
use tower_http::{services::{ServeDir, ServeFile}, trace::TraceLayer};
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use tracing::{info, warn};

pub struct SsoAuth {
    providers: HashMap<String, Box<dyn SsoProvider + Send + Sync>>,
}

struct AppState {
    auth_provider: Arc<DbPrimaryProvider>,
    sso_auth: Arc<SsoAuth>,
    user_store: UserStore,
}

pub fn app_router() -> Router {
    let config = config::load_config().expect("Failed to load configuration");

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let key = Key::generate();

    let user_store = Arc::new(std::sync::Mutex::new(HashMap::new()));
    let auth_provider = Arc::new(DbPrimaryProvider::new(user_store.clone()));

    let mut sso_providers = HashMap::new();
    sso_providers.insert(
        "google".to_string(),
        Box::new(GoogleSsoProvider::new(&config.auth.sso_providers.google).unwrap())
            as Box<dyn SsoProvider + Send + Sync>,
    );
    sso_providers.insert(
        "microsoft".to_string(),
        Box::new(MicrosoftSsoProvider::new(&config.auth.sso_providers.microsoft).unwrap())
            as Box<dyn SsoProvider + Send + Sync>,
    );
    let sso_auth = Arc::new(SsoAuth {
        providers: sso_providers,
    });

    let app_state = Arc::new(AppState {
        auth_provider,
        sso_auth,
        user_store,
    });

    Router::new()
        .route("/api/auth/signup", post(signup))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/user", get(get_user))
        .route("/api/auth/sso/login/{provider}", get(sso_login))
        .route("/api/auth/sso/callback/{provider}", get(sso_callback))
        .route("/ws", get(ws_handler))
        .fallback_service(get_service(
            ServeDir::new("hudwiz/frontend/dist")
                .fallback(ServeFile::new("hudwiz/frontend/dist/index.html")),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(key))
        .layer(CookieManagerLayer::new())
        .layer(session_layer)
        .with_state(app_state)
}

async fn signup(
    State(state): State<Arc<AppState>>,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    info!("Signup attempt for user: {}", creds.username);
    let mut users = state.user_store.lock().unwrap();
    if users.contains_key(&creds.username) {
        warn!("Signup failed for user {}: username already exists", creds.username);
        return StatusCode::CONFLICT;
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(creds.password.as_bytes(), &salt).unwrap().to_string();

    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: creds.username.clone(),
        password_hash,
    };

    users.insert(creds.username.clone(), user);
    info!("Successfully signed up user: {}", creds.username);

    StatusCode::CREATED
}

async fn login(
    session: Session,
    State(state): State<Arc<AppState>>,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    info!("Login attempt for user: {}", creds.username);
    let result = state.auth_provider.login_with_credentials(creds).await;
    match result {
        Ok(user_info) => {
            info!("Login successful for user: {}", user_info.email);
            session.insert("user", user_info.clone()).await.unwrap();
            info!("Session created for user");
            (StatusCode::OK, JsonResponse(user_info)).into_response()
        }
        Err(e) => {
            warn!("Login failed: {}", e);
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    info!("WebSocket connection upgrade requested");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    info!("WebSocket connection established");
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    info!("Logout attempt");
    session.clear().await;
    info!("Session cleared");
    StatusCode::OK
}

async fn get_user(session: Session) -> impl IntoResponse {
    info!("Attempting to retrieve user from session");
    match session.get::<UserInfo>("user").await.unwrap() {
        Some(user) => {
            info!("Found user {} in session", user.email);
            (StatusCode::OK, JsonResponse(user)).into_response()
        },
        None => {
            warn!("No user found in session, returning UNAUTHORIZED");
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}

async fn sso_login(
    Path(provider_name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    info!("Attempting SSO login with provider: {}", provider_name);
    match state.sso_auth.providers.get(&provider_name) {
        Some(p) => p.get_login_redirect().unwrap().into_response(),
        None => {
            warn!("Invalid SSO provider requested: {}", provider_name);
            StatusCode::BAD_REQUEST.into_response()
        }
    }
}

async fn sso_callback(
    Path(provider_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
    session: Session,
) -> impl IntoResponse {
    info!("Handling SSO callback for provider: {}", provider_name);
    let code = params.get("code").unwrap().to_string();
    let state_param = params.get("state").unwrap().to_string();
    match state.sso_auth.providers.get(&provider_name) {
        Some(p) => {
            let user_info = p.handle_callback(code, state_param).await.unwrap();
            info!("SSO callback successful for user: {}", user_info.email);
            session.insert("user", user_info).await.unwrap();
            info!("Session created for SSO user");
            StatusCode::OK.into_response()
        }
        None => {
            warn!("Invalid SSO provider in callback: {}", provider_name);
            StatusCode::BAD_REQUEST.into_response()
        }
    }
}
