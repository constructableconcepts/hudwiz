use gloo_console::{error, log};
use serde::Serialize;
use std::rc::Rc;
use yew::prelude::*;
use crate::services::api_service::{ApiService, ApiServiceEnum};
use wasm_bindgen_futures::spawn_local;
use crate::auth::UserInfo;

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub user: Option<UserInfo>,
    pub login: Callback<(String, String)>,
    pub logout: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let api_service = use_context::<Rc<ApiServiceEnum>>().expect("no api service found");
    let user = use_state(|| None);
    log!("AuthProvider mounted");

    {
        let user = user.clone();
        let api_service = api_service.clone();
        use_effect_with((), move |_| {
            log!("AuthProvider use_effect_with running");
            let user = user.clone();
            let api_service = api_service.clone();
            spawn_local(async move {
                log!("AuthProvider fetching user from /api/auth/user");
                match api_service.get("/api/auth/user").await {
                    Ok(u) => {
                        log!("AuthProvider successfully fetched user");
                        user.set(Some(u))
                    },
                    Err(_) => {
                        log!("AuthProvider failed to fetch user, setting user to None");
                        user.set(None)
                    },
                }
            });
            || ()
        });
    }

    let login = {
        let user = user.clone();
        let api_service = api_service.clone();
        Callback::from(move |(username, password)| {
            let user = user.clone();
            let api_service = api_service.clone();
            log!("auth_service: login callback triggered.");
            spawn_local(async move {
                log!("auth_service: spawning local task for login API call.");
                let request = LoginRequest { username, password };
                let location = web_sys::window().unwrap().location();
                let base_url = format!("{}//{}", location.protocol().unwrap(), location.host().unwrap());
                match api_service.post::<_, UserInfo>(format!("{}/api/auth/login", base_url).as_str(), request).await {
                    Ok(user_info) => {
                        log!("auth_service: login POST successful.");
                        user.set(Some(user_info));
                    }
                    Err(e) => {
                        error!(format!("auth_service: login POST request failed: {}", e));
                        user.set(None);
                    }
                }
            });
        })
    };

    let logout = {
        let user = user.clone();
        let api_service = api_service.clone();
        Callback::from(move |_| {
            let user = user.clone();
            let api_service = api_service.clone();
            spawn_local(async move {
                let _ : Result<(), _> = api_service.post("/api/auth/logout", &()).await;
                user.set(None);
            });
        })
    };

    let auth_context = AuthContext {
        user: (*user).clone(),
        login,
        logout,
    };

    html! {
        <ContextProvider<AuthContext> context={auth_context}>
            { for props.children.iter() }
        </ContextProvider<AuthContext>>
    }
}
