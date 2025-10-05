use async_trait::async_trait;
use axum::response::Redirect;
use serde::{Deserialize, Serialize};
use anyhow::bail;
use argon2::{password_hash::PasswordVerifier, Argon2};
use tracing::{info, warn, error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

pub type UserStore = Arc<Mutex<HashMap<String, User>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait AuthProvider {
    /// Authenticates a user with a username and password.
    async fn login_with_credentials(&self, creds: Credentials) -> Result<UserInfo, anyhow::Error>;

    /// Returns a redirect response to an SSO provider's login page.
    fn get_sso_login_redirect(&self, provider_name: &str) -> Result<Redirect, anyhow::Error>;

    /// Handles the callback from an SSO provider after user login.
    async fn handle_sso_callback(
        &self,
        provider_name: &str,
        code: String,
        state: String,
    ) -> Result<UserInfo, anyhow::Error>;

    /// Logs the user out.
    fn logout(&self) -> Result<Redirect, anyhow::Error>;
}

pub struct DbPrimaryProvider {
    user_store: UserStore,
}

impl DbPrimaryProvider {
    pub fn new(user_store: UserStore) -> Self {
        Self { user_store }
    }
}

#[async_trait]
impl AuthProvider for DbPrimaryProvider {
    async fn login_with_credentials(&self, creds: Credentials) -> Result<UserInfo, anyhow::Error> {
        info!("Attempting to log in user: {}", creds.username);

        let users = self.user_store.lock().unwrap();
        let user = match users.get(&creds.username) {
            Some(user) => user,
            None => {
                warn!("Invalid username provided: {}", creds.username);
                bail!("Invalid username");
            }
        };

        info!("Username matched. Proceeding to password verification.");
        let password_hash = &user.password_hash;

        let parsed_hash = match argon2::PasswordHash::new(password_hash) {
            Ok(hash) => {
                info!("Password hash parsed successfully from store.");
                hash
            },
            Err(e) => {
                error!("Failed to parse password hash from store: {}", e);
                bail!("Server configuration error: Invalid password hash format");
            }
        };

        info!("Verifying password for user: {}", creds.username);
        if Argon2::default().verify_password(creds.password.as_bytes(), &parsed_hash).is_ok() {
            info!("Password verification successful for user: {}", creds.username);
            Ok(UserInfo {
                id: user.id.clone(),
                name: user.username.clone(),
                email: user.username.clone(),
            })
        } else {
            warn!("Invalid password for user: {}", creds.username);
            bail!("Invalid password");
        }
    }

    fn get_sso_login_redirect(&self, _provider_name: &str) -> Result<Redirect, anyhow::Error> {
        unimplemented!()
    }

    async fn handle_sso_callback(
        &self,
        _provider_name: &str,
        _code: String,
        _state: String,
    ) -> Result<UserInfo, anyhow::Error> {
        unimplemented!()
    }

    fn logout(&self) -> Result<Redirect, anyhow::Error> {
        Ok(Redirect::to("/"))
    }
}


#[cfg(test)]
mod tests {
    use argon2::{
        password_hash::{
            rand_core::OsRng,
            PasswordHasher, SaltString
        },
        Argon2
    };
    use tracing::info;

    #[test]
    fn generate_password_hash() {
        let password = b"password";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
        info!("START_HASH:{}END_HASH", password_hash);
    }
}
