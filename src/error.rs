use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("Failed to verify password")]
    VerifyError,
    #[error("Failed to hash password")]
    HashError,
    #[error("Password must be 8-64 characters")]
    InvalidPassword,
}

impl ResponseError for ControllerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(json!({ "error": self.to_string() }).to_string())
    }
}
