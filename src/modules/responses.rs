use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

use crate::modules::errors::ServiceError;

#[derive(Serialize)]
pub struct ApiError {
    status: u16,
    pub error: String,
    pub details: Option<String>,
}

impl ApiError {
    pub fn new(status: StatusCode, error: String, details: Option<String>) -> Self {
        Self {
            status: status.into(),
            details,
            error,
        }
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        Self {
            status: err.status.into(),
            error: err.message,
            details: err.details,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or_default();
        (status, Json(self)).into_response()
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(error: ValidationErrors) -> Self {
        Self {
            details: Some(error.to_string()),
            error: "The request body is malformated".to_string(),
            status: StatusCode::NOT_FOUND.as_u16(),
        }
    }
}
