use crate::modules::errors::ServiceError;
use crate::modules::responses::ApiError;
use axum::Json;
use axum::response::Result;

pub type ApiResponse<T> = Result<Json<T>, ApiError>;
pub type ServiceResult<T> = std::result::Result<T, ServiceError>;
