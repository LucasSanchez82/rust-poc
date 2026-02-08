use axum::extract::State;
use axum::routing::{delete, get, post};
use axum::{Json, Router};

use crate::modules::responses::ApiError;
use crate::modules::states::AppState;
use crate::modules::types::ApiResponse;
use crate::modules::user::dto::{LoginDto, UserDto};
use crate::modules::user::payload::{CreateUser, DeleteUser, LoginPayload};
use crate::modules::user::service::{Service, UserService};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_users))
        .route("/", post(handle_create_user))
        .route("/", delete(handle_delete_user))
}

async fn handle_get_users(State(state): State<AppState>) -> ApiResponse<Box<[UserDto]>> {
    let user_svc = UserService::new(state.connection);
    user_svc.get_all().await.map(Json).map_err(ApiError::from)
}

async fn handle_delete_user(
    State(state): State<AppState>,
    Json(payload): Json<DeleteUser>,
) -> ApiResponse<UserDto> {
    let user_svc = UserService::new(state.connection);
    user_svc
        .delete(payload.id)
        .await
        .map(Json)
        .map_err(ApiError::from)
}

async fn handle_create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> ApiResponse<UserDto> {
    let user_svc = UserService::new(state.connection);
    user_svc
        .create(payload)
        .await
        .map(Json)
        .map_err(ApiError::from)
}

pub async fn handle_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> ApiResponse<LoginDto> {
    let user_svc = UserService::new(state.connection);

    user_svc
        .login(payload)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
