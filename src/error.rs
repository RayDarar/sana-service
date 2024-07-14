use actix_web::HttpResponse;
use actix_web::{error, http::StatusCode};
use derive_more::Error;

use derive_more::Display;
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum SanaError {
    #[display(fmt = "{}: {}", _0, _1)]
    Generic(StatusCode, String),
    ValidationError,
    InternalError,
}

impl error::ResponseError for SanaError {
    fn status_code(&self) -> StatusCode {
        match *self {
            SanaError::Generic(code, _) => code,
            SanaError::ValidationError => StatusCode::BAD_REQUEST,
            SanaError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            SanaError::Generic(code, message) => HttpResponse::build(*code).json(json!({
              "message": message
            })),
            SanaError::ValidationError => HttpResponse::build(self.status_code()).json(json!({
              "message": "Validation error"
            })),
            SanaError::InternalError => HttpResponse::build(self.status_code()).json(json!({
              "message": "Internal server error"
            })),
        }
    }
}
