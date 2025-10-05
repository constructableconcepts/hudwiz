use super::transport::Transport;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures_channel::oneshot;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebSocket, MessageEvent, ErrorEvent};

pub struct WebSocketTransport {
    ws: WebSocket,
}

#[async_trait(?Send)]
impl Transport for WebSocketTransport {
    async fn connect(server_url: &str) -> Result<Self> {
        let ws = WebSocket::new(server_url)
            .map_err(|e| anyhow!("Failed to create WebSocket: {:?}", e))?;

        let (open_tx, open_rx) = oneshot::channel::<Result<()>>();
        let mut open_tx = Some(open_tx);

        // onopen handler
        let onopen_callback = Closure::wrap(Box::new(move || {
            log::info!("WebSocket connection established.");
            if let Some(tx) = open_tx.take() {
                let _ = tx.send(Ok(()));
            }
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        // onerror handler
        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            log::error!("WebSocket error: {:?}", e.message());
        }) as Box<dyn FnMut(_)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        // onclose handler
        let onclose_callback = Closure::wrap(Box::new(move || {
            log::info!("WebSocket connection closed.");
        }) as Box<dyn FnMut()>);
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        // onmessage handler (dummy)
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                log::info!("Dummy onmessage: {:?}", txt);
            }
        }) as Box<dyn FnMut(_)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        open_rx.await??;

        Ok(Self { ws })
    }

    async fn send(&self, message: &str) -> Result<()> {
        self.ws
            .send_with_str(message)
            .map_err(|e| anyhow!("Failed to send message: {:?}", e))?;
        Ok(())
    }
}