use anyhow::Result;
use std::time::Duration;
use tracing::{error, info, info_span, Instrument};
use wtransport::{endpoint::IncomingSession, Endpoint, Identity, ServerConfig};

pub async fn start_webtransport_server() -> Result<()> {
    let config = ServerConfig::builder()
        .with_bind_address("0.0.0.0:4433".parse()?)
        .with_identity(Identity::self_signed(["localhost"])?)
        .keep_alive_interval(Some(Duration::from_secs(3)))
        .build();

    let server = Endpoint::server(config)?;

    info!("WebTransport server listening on port 4433");

    for id in 0.. {
        let incoming_session = server.accept().await;
        tokio::spawn(
            handle_connection(incoming_session).instrument(info_span!("Connection", id)),
        );
    }

    Ok(())
}

async fn handle_connection(incoming_session: IncomingSession) {
    if let Err(err) = handle_connection_impl(incoming_session).await {
        error!("Error handling connection: {:?}", err);
    }
}

async fn handle_connection_impl(incoming_session: IncomingSession) -> Result<()> {
    let mut buffer = vec![0; 65536].into_boxed_slice();

    info!("Waiting for session request...");

    let session_request = incoming_session.await?;

    info!(
        "New session: Authority: '{}', Path: '{}'",
        session_request.authority(),
        session_request.path()
    );

    let connection = session_request.accept().await?;

    info!("Connection accepted. Waiting for data from client...");

    loop {
        tokio::select! {
            stream = connection.accept_bi() => {
                let mut stream = stream?;
                info!("Accepted BI stream");

                if let Some(bytes_read) = stream.1.read(&mut buffer).await? {
                    let str_data = std::str::from_utf8(&buffer[..bytes_read])?;
                    info!("Received (bi) '{str_data}' from client");
                    stream.0.write_all(b"ACK").await?;
                }
            }
            stream = connection.accept_uni() => {
                let mut stream = stream?;
                info!("Accepted UNI stream");

                if let Some(bytes_read) = stream.read(&mut buffer).await? {
                    let str_data = std::str::from_utf8(&buffer[..bytes_read])?;
                    info!("Received (uni) '{str_data}' from client");
                    let mut ack_stream = connection.open_uni().await?.await?;
                    ack_stream.write_all(b"ACK").await?;
                }
            }
            dgram = connection.receive_datagram() => {
                let dgram = dgram?;
                let str_data = std::str::from_utf8(&dgram)?;
                info!("Received (dgram) '{str_data}' from client");
                connection.send_datagram(b"ACK")?;
            }
        }
    }
}
