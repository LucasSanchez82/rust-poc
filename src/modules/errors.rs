use std::error::Error;

use axum::http::StatusCode;
use sea_orm::DbErr;
use tracing::warn;
use validator::ValidationError;

#[derive(Debug)]
pub struct ServiceError {
    pub status: StatusCode,
    pub message: String,
    pub details: Option<String>,
}

impl ServiceError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn internal(message: impl Into<String> + tracing::Value) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ServiceError {}

impl From<ServiceError> for (StatusCode, String) {
    fn from(err: ServiceError) -> Self {
        (err.status, err.message)
    }
}

impl From<DbErr> for ServiceError {
    fn from(value: DbErr) -> Self {
        warn!("DB ERROR: {:#?}", value);
        Self::internal("Internal Error during the processing of your request...")
    }
}
