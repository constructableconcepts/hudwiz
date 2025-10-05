use super::transport::Transport;
use super::websocket_transport::WebSocketTransport;
use super::webtransport_transport::WebTransportTransport;
use anyhow::Result;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveTransport {
    WebTransport,
    WebSocket,
}

pub struct TransportManager {
    // No-op
}

impl TransportManager {
    pub async fn new(server_url: &str) -> Result<(Rc<dyn Transport>, ActiveTransport)> {
        match WebTransportTransport::connect(server_url).await {
            Ok(transport) => {
                log::info!("Successfully connected using WebTransport.");
                Ok((Rc::new(transport), ActiveTransport::WebTransport))
            }
            Err(e) => {
                log::warn!("WebTransport connection failed: {:?}. Falling back to WebSocket.", e);
                match WebSocketTransport::connect(server_url).await {
                    Ok(transport) => {
                        log::info!("Successfully connected using WebSocket.");
                        Ok((Rc::new(transport), ActiveTransport::WebSocket))
                    }
                    Err(e_ws) => {
                        log::error!("WebSocket connection also failed: {:?}", e_ws);
                        Err(e_ws)
                    }
                }
            }
        }
    }
}