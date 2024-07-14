use std::env;

use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Result;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

mod error;
mod health;

pub struct AppState {}

fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::register());
}

#[actix_web::main]
async fn main() -> Result<()> {
    let tracing_layer = tracing_subscriber::fmt::layer().json().boxed();
    tracing_subscriber::registry().with(tracing_layer).init();

    let port = match env::var("PORT") {
        Ok(value) => value.parse::<u16>()?,
        Err(_) => 3000,
    };

    let state = web::Data::new(AppState {});

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/api").configure(config_routes))
            .default_service(web::to(HttpResponse::NotFound))
    })
    .workers(4)
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
