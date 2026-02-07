use crate::modules::responses::ApiError;
use axum::Json;
use axum::response::Result;

pub type ApiResponse<T> = Result<Json<T>, ApiError>;
