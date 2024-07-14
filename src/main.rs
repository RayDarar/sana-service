use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
use anyhow::Result;

use serde_json::json;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

struct AppState {}

#[actix_web::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let port = match env::var("PORT") {
        Ok(value) => value.parse::<u16>()?,
        Err(_) => 3000,
    };

    let state = web::Data::new(AppState {});

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/api").service(health))
    })
    .workers(4)
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}

#[get("/health")]
async fn health() -> impl Responder {
    web::Json(json!({
        "status": true
    }))
}
