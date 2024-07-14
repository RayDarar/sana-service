use actix_web::{web, Responder, Scope};
use serde_json::json;

pub fn register() -> Scope {
    web::scope("/v1/health").route("status", web::get().to(v1_status))
}

async fn v1_status() -> impl Responder {
    web::Json(json!({
        "status": true
    }))
}
