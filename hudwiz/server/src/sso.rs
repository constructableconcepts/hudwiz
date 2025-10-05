use crate::config::SsoProviderConfig;
use async_trait::async_trait;
use axum::response::Redirect;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;

#[async_trait]
pub trait SsoProvider {
    fn get_login_redirect(&self) -> Result<Redirect, anyhow::Error>;
    async fn handle_callback(&self, code: String, state: String) -> Result<super::auth::UserInfo, anyhow::Error>;
}

pub struct GoogleSsoProvider {
    client: BasicClient,
}

impl GoogleSsoProvider {
    pub fn new(config: &SsoProviderConfig) -> Result<Self, anyhow::Error> {
        let client_id = ClientId::new(config.client_id.clone());
        let client_secret = ClientSecret::new(config.client_secret.clone());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())?;
        let redirect_url = RedirectUrl::new(config.redirect_uri.clone())?;

        let client = BasicClient::new(
            client_id,
            Some(client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        Ok(Self { client })
    }
}

#[async_trait]
impl SsoProvider for GoogleSsoProvider {
    fn get_login_redirect(&self) -> Result<Redirect, anyhow::Error> {
        let (authorize_url, _csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .url();

        Ok(Redirect::to(authorize_url.as_ref()))
    }

    async fn handle_callback(
        &self,
        code: String,
        _state: String,
    ) -> Result<super::auth::UserInfo, anyhow::Error> {
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await?;

        let user_info: GoogleUserInfo = reqwest::Client::new()
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token.access_token().secret())
            .send()
            .await?
            .json()
            .await?;

        Ok(super::auth::UserInfo {
            id: user_info.sub,
            name: user_info.name,
            email: user_info.email,
        })
    }
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    sub: String,
    name: String,
    email: String,
}

pub struct MicrosoftSsoProvider {
    client: BasicClient,
}

impl MicrosoftSsoProvider {
    pub fn new(config: &SsoProviderConfig) -> Result<Self, anyhow::Error> {
        let client_id = ClientId::new(config.client_id.clone());
        let client_secret = ClientSecret::new(config.client_secret.clone());
        let auth_url = AuthUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string())?;
        let token_url = TokenUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string())?;
        let redirect_url = RedirectUrl::new(config.redirect_uri.clone())?;

        let client = BasicClient::new(
            client_id,
            Some(client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        Ok(Self { client })
    }
}

#[async_trait]
impl SsoProvider for MicrosoftSsoProvider {
    fn get_login_redirect(&self) -> Result<Redirect, anyhow::Error> {
        let (authorize_url, _csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("User.Read".to_string()))
            .url();

        Ok(Redirect::to(authorize_url.as_ref()))
    }

    async fn handle_callback(
        &self,
        code: String,
        _state: String,
    ) -> Result<super::auth::UserInfo, anyhow::Error> {
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await?;

        let user_info: MicrosoftUserInfo = reqwest::Client::new()
            .get("https://graph.microsoft.com/v1.0/me")
            .bearer_auth(token.access_token().secret())
            .send()
            .await?
            .json()
            .await?;

        Ok(super::auth::UserInfo {
            id: user_info.id,
            name: user_info.display_name,
            email: user_info.mail.or(user_info.user_principal_name).unwrap_or_default(),
        })
    }
}

#[derive(Deserialize)]
struct MicrosoftUserInfo {
    id: String,
    #[serde(rename = "displayName")]
    display_name: String,
    mail: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: Option<String>,
}
