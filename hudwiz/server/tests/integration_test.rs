use axum::http::StatusCode;
use server_lib::{app_router, auth::Credentials};
use tokio::net::TcpListener;

#[tokio::test]
async fn test_sso_login() {
    let app = app_router();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // Test Google SSO login
    let res = client
        .get(format!("http://{}/api/auth/sso/login/google", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::SEE_OTHER);

    // Test Microsoft SSO login
    let res = client
        .get(format!("http://{}/api/auth/sso/login/microsoft", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn test_login() {
    let app = app_router();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();

    // Test successful login
    let res = client
        .post(format!("http://{}/api/auth/login", addr))
        .json(&Credentials {
            username: "testuser".to_string(),
            password: "password".to_string(),
        })
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Test failed login
    let res = client
        .post(format!("http://{}/api/auth/login", addr))
        .json(&Credentials {
            username: "testuser".to_string(),
            password: "wrongpassword".to_string(),
        })
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
