use super::transport::Transport;
use super::transport_manager::{TransportManager, ActiveTransport};
use anyhow::Result;
use std::rc::Rc;

pub struct RealtimeService {
    transport: Rc<dyn Transport>,
    pub active_transport: ActiveTransport,
}

impl PartialEq for RealtimeService {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.transport, &other.transport) && self.active_transport == other.active_transport
    }
}

impl RealtimeService {
    pub async fn new(server_url: &str) -> Result<Self> {
        let (transport, active_transport) = TransportManager::new(server_url).await?;
        Ok(Self { transport, active_transport })
    }

    pub async fn send(&self, message: &str) -> Result<()> {
        self.transport.send(message).await
    }
}