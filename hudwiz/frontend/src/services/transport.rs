use anyhow::Result;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Transport {
    async fn connect(server_url: &str) -> Result<Self>
    where
        Self: Sized;
    async fn send(&self, message: &str) -> Result<()>;
}