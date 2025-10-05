use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;
use std::pin::Pin;
use yew::prelude::*;
use std::rc::Rc;
use gloo_storage::{LocalStorage, Storage};
use reqwest::Response;

const TOKEN_KEY: &str = "hudwiz_token";

pub trait ApiService {
    fn get<'a, T: DeserializeOwned + 'static>(&'a self, endpoint: &'a str) -> Pin<Box<dyn Future<Output = Result<T>> + 'a>>;
    fn post<'a, B: Serialize + 'static, T: DeserializeOwned + 'static>(&'a self, endpoint: &'a str, body: B) -> Pin<Box<dyn Future<Output = Result<T>> + 'a>>;
    fn post_no_response<'a, B: Serialize + 'static>(&'a self, endpoint: &'a str, body: B) -> Pin<Box<dyn Future<Output = Result<Response>> + 'a>>;
}

#[derive(Clone)]
pub struct MockApiService;

impl ApiService for MockApiService {
    fn get<'a, T: DeserializeOwned + 'static>(&'a self, endpoint: &'a str) -> Pin<Box<dyn Future<Output = Result<T>> + 'a>> {
        Box::pin(async move {
            if endpoint == "/api/auth/user" {
                let mock_user = serde_json::json!({
                    "id": "mock_user_id",
                    "name": "Mock User",
                    "email": "testuser@fakedomain.com"
                });
                let mock_json = serde_json::from_value(mock_user)?;
                Ok(mock_json)
            } else {
                let mock_json = serde_json::from_str("{}")?;
                Ok(mock_json)
            }
        })
    }

    fn post<'a, B: Serialize + 'static, T: DeserializeOwned + 'static>(&'a self, endpoint: &'a str, _body: B) -> Pin<Box<dyn Future<Output = Result<T>> + 'a>> {
        Box::pin(async move {
            if endpoint == "/api/auth/login" {
                let response = serde_json::json!({
                    "token": "mock_token"
                });
                let mock_json = serde_json::from_value(response)?;
                Ok(mock_json)
            } else {
                let mock_json = serde_json::from_str("{}")?;
                Ok(mock_json)
            }
        })
    }

    fn post_no_response<'a, B: Serialize + 'static>(&'a self, _endpoint: &'a str, _body: B) -> Pin<Box<dyn Future<Output = Result<Response>> + 'a>> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct RealApiService;

impl ApiService for RealApiService {
    fn get<'a, T: DeserializeOwned + 'static>(&'a self, endpoint: &'a str) -> Pin<Box<dyn Future<Output = Result<T>> + 'a>> {
        Box::pin(async move {
            let client = reqwest::Client::new();
            let mut request = client.get(endpoint);
            if let Ok(token) = LocalStorage::get::<String>(TOKEN_KEY) {
                request = request.bearer_auth(token);
            }
            let response = request.send().await?;
            let json = response.json::<T>().await?;
            Ok(json)
        })
    }

    fn post<'a, B: Serialize + 'static, T: DeserializeOwned + 'static>(&'a self, endpoint: &'a str, body: B) -> Pin<Box<dyn Future<Output = Result<T>> + 'a>> {
        Box::pin(async move {
            let client = reqwest::Client::new();
            let mut request = client.post(endpoint).json(&body);
            if let Ok(token) = LocalStorage::get::<String>(TOKEN_KEY) {
                request = request.bearer_auth(token);
            }
            let response = request.send().await?;
            let json = response.json::<T>().await?;
            Ok(json)
        })
    }

    fn post_no_response<'a, B: Serialize + 'static>(&'a self, endpoint: &'a str, body: B) -> Pin<Box<dyn Future<Output = Result<Response>> + 'a>> {
        Box::pin(async move {
            let client = reqwest::Client::new();
            let mut request = client.post(endpoint).json(&body);
            if let Ok(token) = LocalStorage::get::<String>(TOKEN_KEY) {
                request = request.bearer_auth(token);
            }
            let response = request.send().await?;
            Ok(response)
        })
    }
}

#[derive(Clone)]
pub enum ApiServiceEnum {
    Mock(MockApiService),
    Real(RealApiService),
}

impl ApiService for ApiServiceEnum {
    fn get<'a, T: DeserializeOwned + 'static>(&'a self, endpoint: &'a str) -> Pin<Box<dyn Future<Output = Result<T>> + 'a>> {
        match self {
            ApiServiceEnum::Mock(mock) => mock.get(endpoint),
            ApiServiceEnum::Real(real) => real.get(endpoint),
        }
    }

    fn post<'a, B: Serialize + 'static, T: DeserializeOwned + 'static>(&'a self, endpoint: &'a str, body: B) -> Pin<Box<dyn Future<Output = Result<T>> + 'a>> {
        match self {
            ApiServiceEnum::Mock(mock) => mock.post(endpoint, body),
            ApiServiceEnum::Real(real) => real.post(endpoint, body),
        }
    }

    fn post_no_response<'a, B: Serialize + 'static>(&'a self, endpoint: &'a str, body: B) -> Pin<Box<dyn Future<Output = Result<Response>> + 'a>> {
        match self {
            ApiServiceEnum::Mock(mock) => mock.post_no_response(endpoint, body),
            ApiServiceEnum::Real(real) => real.post_no_response(endpoint, body),
        }
    }
}

// Manually implement PartialEq for the enum
impl PartialEq for ApiServiceEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ApiServiceEnum::Mock(_), ApiServiceEnum::Mock(_)) => true,
            (ApiServiceEnum::Real(_), ApiServiceEnum::Real(_)) => true,
            _ => false,
        }
    }
}


#[derive(Properties, PartialEq)]
pub struct ApiProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ApiProvider)]
pub fn api_provider(props: &ApiProviderProps) -> Html {
    let api_service: Rc<ApiServiceEnum> = if cfg!(feature = "mock_api") {
        Rc::new(ApiServiceEnum::Mock(MockApiService))
    } else {
        Rc::new(ApiServiceEnum::Real(RealApiService))
    };

    html! {
        <ContextProvider<Rc<ApiServiceEnum>> context={api_service}>
            { for props.children.iter() }
        </ContextProvider<Rc<ApiServiceEnum>>>
    }
}
