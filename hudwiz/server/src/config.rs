use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    pub primary_provider: String,
    pub providers: Providers,
    pub sso_providers: SsoProviders,
}

#[derive(Debug, Deserialize)]
pub struct Providers {
    pub mock: MockProviderConfig,
}

#[derive(Debug, Deserialize)]
pub struct MockProviderConfig {
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct SsoProviders {
    pub google: SsoProviderConfig,
    pub microsoft: SsoProviderConfig,
    pub apple: SsoProviderConfig,
}

#[derive(Debug, Deserialize)]
pub struct SsoProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub fn load_config() -> Result<Config, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("server_config"))
        .build()?;
    settings.try_deserialize()
}
