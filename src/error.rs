use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use sea_orm::DbErr;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(DbErr),
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::DatabaseError(err) => write!(f, "Database error: {}", err),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> ApiError {
        ApiError::DatabaseError(err)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    detail: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            detail: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
