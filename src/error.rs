use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ApiError::InvalidSignature => {
                actix_web::HttpResponse::BadRequest().json(format!("{}", self))
            }
            _ => actix_web::HttpResponse::InternalServerError().json(format!("{}", self)),
        }
    }
}
