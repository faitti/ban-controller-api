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
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Failed to generate apikey")]
    GenerationFailure,
    #[error("0 rows updated")]
    InsertError,
    #[error("Empty struct not allowed")]
    EmptyStruct,
    #[error("Failed to insert data into database: `{0}`")]
    DieselError(#[from] diesel::result::Error),
}

unsafe impl Send for ControllerError {}

impl ResponseError for ControllerError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InvalidPassword | Self::EmptyStruct => StatusCode::NOT_ACCEPTABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(json!({ "error": self.to_string() }).to_string())
    }
}
