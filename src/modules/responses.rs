use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    status: u16,
    pub error: String,
    pub details: Option<String>,
}

impl ApiError {
    pub fn new(status: StatusCode, error: String, details: Option<String>) -> Self {
        return Self {
            status: status.into(),
            details,
            error,
        };
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or_default();
        (status, Json(self)).into_response()
    }
}
