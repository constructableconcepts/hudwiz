use server_lib::{app_router, webtransport::start_webtransport_server};
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{prelude::*, EnvFilter, fmt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();

    tokio::spawn(async {
        if let Err(e) = start_webtransport_server().await {
            eprintln!("Failed to start WebTransport server: {}", e);
        }
    });

    info!("Server starting...");
    let app = app_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("listening on http://{}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            eprintln!("Error: Address {} is already in use. Is another instance of the server running?", addr);
            std::process::exit(1);
        }
        Err(e) => panic!("Failed to bind to address {}: {}", addr, e),
    };
    axum::serve(listener, app).await.unwrap();
}
