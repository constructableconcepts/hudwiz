use super::transport::Transport;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures_channel::oneshot;
use std::cell::RefCell;
use yew::Callback;
use yew_webtransport::webtransport::{WebTransportService, WebTransportStatus, WebTransportTask};
use web_sys::{WebTransportReceiveStream, WebTransportBidirectionalStream};

pub struct WebTransportTransport {
    task: WebTransportTask,
}

#[async_trait(?Send)]
impl Transport for WebTransportTransport {
    async fn connect(server_url: &str) -> Result<Self> {
        let (open_tx, open_rx) = oneshot::channel::<Result<WebTransportTask>>();
        let open_tx = RefCell::new(Some(open_tx)); // Wrap in RefCell

        let on_datagram = Callback::from(|data: Vec<u8>| {
            log::info!("Dummy on_datagram: {:?}", data);
        });

        let on_unidirectional_stream = Callback::from(|stream: WebTransportReceiveStream| {
            log::info!("Dummy on_unidirectional_stream: {:?}", stream);
        });

        let on_bidirectional_stream = Callback::from(|stream: WebTransportBidirectionalStream| {
            log::info!("Dummy on_bidirectional_stream: {:?}", stream);
        });

        let server_url_clone = server_url.to_string();
        let notification = Callback::from(move |status: WebTransportStatus| {
            if let Some(tx) = open_tx.borrow_mut().take() { // Use borrow_mut()
                match status {
                    WebTransportStatus::Opened => {
                        let task_result = WebTransportService::connect(
                            &server_url_clone,
                            Callback::from(|_: Vec<u8>| {}),
                            Callback::from(|_: WebTransportReceiveStream| {}),
                            Callback::from(|_: WebTransportBidirectionalStream| {}),
                            Callback::from(|_: WebTransportStatus| {}),
                        ).map_err(|e| anyhow!("Failed to reconnect for task: {:?}", e));
                        let _ = tx.send(task_result);
                    }
                    WebTransportStatus::Closed(reason) | WebTransportStatus::Error(reason) => {
                        let _ = tx.send(Err(anyhow!("Connection failed: {:?}", reason)));
                    }
                }
            }
        });

        let _task = WebTransportService::connect(
            server_url,
            on_datagram,
            on_unidirectional_stream,
            on_bidirectional_stream,
            notification,
        ).map_err(|e| anyhow!("Failed to initiate connection: {:?}", e))?;

        let task = open_rx.await??;
        Ok(Self { task })
    }

    async fn send(&self, message: &str) -> Result<()> {
        WebTransportTask::send_datagram(self.task.transport.clone(), message.as_bytes().to_vec());
        Ok(())
    }
}