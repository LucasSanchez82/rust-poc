use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use validator::Validate;

use crate::modules::errors::ServiceError;
use crate::modules::responses::ApiError;
use crate::modules::states::AppState;
use crate::modules::types::ApiResponse;
use crate::modules::user::dto::UserDto;
use crate::modules::user::payload::CreateUser;
use crate::modules::user::service::UserService;
use crate::utils::extractor::ExtractValidated;

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_users))
        .route("/", post(handle_create_user))
        .route("/initial_root_user", post(handle_create_initial_root_user))
        .route("/{id}", delete(handle_delete_user))
}

async fn handle_create_initial_root_user(
    state: State<AppState>,
    ExtractValidated(payload): ExtractValidated<CreateUser>,
) -> ApiResponse<UserDto> {
    let user_svc = UserService::new(&state.connection);
    user_svc
        .create_initial_root_user(payload)
        .await
        .map(Json)
        .map_err(ApiError::from)
}

async fn handle_get_users(State(state): State<AppState>) -> ApiResponse<Vec<UserDto>> {
    let user_svc = UserService::new(&state.connection);
    user_svc.get_all().await.map(Json).map_err(ApiError::from)
}

async fn handle_delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResponse<UserDto> {
    let user_svc = UserService::new(&state.connection);
    user_svc.delete(id).await.map(Json).map_err(ApiError::from)
}

async fn handle_create_user(
    State(state): State<AppState>,
    ExtractValidated(payload): ExtractValidated<CreateUser>,
) -> ApiResponse<UserDto> {
    let user_svc = UserService::new(&state.connection);
    user_svc
        .create(payload)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
