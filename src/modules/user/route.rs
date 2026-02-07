use axum::extract::State;
use axum::routing::{delete, get, post};
use axum::{Json, Router, extract};

use crate::modules::states::AppState;
use crate::modules::types::ApiResponse;
use crate::modules::user::dto::UserDto;
use crate::modules::user::payload::{CreateUser, DeleteUser};
use crate::modules::user::service::Service;
use crate::modules::user::service::UserService;

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_users))
        .route("/", post(handle_create_user))
        .route("/", delete(handle_delete_user))
}

async fn handle_get_users(State(state): State<AppState>) -> ApiResponse<Box<[UserDto]>> {
    let user_svc = UserService::new(state.connection);
    
    user_svc.get_all().await
}

async fn handle_delete_user(
    State(state): State<AppState>,

    Json(payload): Json<DeleteUser>,
) -> ApiResponse<UserDto> {
    let user_svc = UserService::new(state.connection);
    

    user_svc.delete(payload.id).await
}

async fn handle_create_user(
    State(state): State<AppState>,

    extract::Json(payload): extract::Json<CreateUser>,
) -> ApiResponse<UserDto> {
    let user_svc = UserService::new(state.connection);
    
    user_svc.create(payload).await
}
