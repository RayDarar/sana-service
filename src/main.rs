use std::env;

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use tokio::{net::TcpListener, signal};

use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let port = match env::var("PORT") {
        Ok(value) => value.parse::<i32>()?,
        Err(_) => 3000,
    };

    tokio::spawn(serve_app(port));

    info!(port, "Server is started");

    match signal::ctrl_c().await {
        Ok(()) => info!("Graceful shutdown"),
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    Ok(())
}

async fn serve_app(port: i32) -> Result<()> {
    let router = Router::new().route("/", get(|| async { "Hello, World!" }));

    let app = Router::new().nest("/api", router);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .with_context(|| format!("Failed to open listener on port {}", port))?;
    axum::serve(listener, app).await?;

    Ok(())
}
