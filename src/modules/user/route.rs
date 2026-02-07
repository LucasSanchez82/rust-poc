use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Result;
use axum::routing::{delete, get, post};
use axum::{Json, Router, extract};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter};
use serde::Deserialize;

use crate::modules::models::entities::user::ActiveModel as UserActiveModel;
use crate::modules::models::entities::user::Column as UserColumn;
use crate::modules::models::entities::user::Entity as UserEntity;
use crate::modules::responses::ApiError;
use crate::modules::states::AppState;
use crate::modules::user::dto::UserDto;
use crate::modules::user::service::Service;
use crate::modules::user::service::UserService;
use sea_orm::EntityTrait;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct DeleteUser {
    id: i32,
}

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_users))
        .route("/", post(handle_create_user))
        .route("/", delete(handle_delete_user))
}

async fn handle_get_users(State(state): State<AppState>) -> Json<Box<[UserDto]>> {
    let user_svc = UserService::new(state.connection);
    let users = user_svc.get_all().await;
    Json(users)
}

async fn handle_delete_user(
    State(state): State<AppState>,

    Json(payload): Json<DeleteUser>,
) -> Result<Json<UserDto>, ApiError> {
    let user_svc = UserService::new(state.connection);

    user_svc.delete(payload.id).await
}

async fn handle_create_user(
    State(state): State<AppState>,

    extract::Json(payload): extract::Json<CreateUser>,
) -> Json<UserDto> {
    let db = state.connection.as_ref();

    let new_user = UserActiveModel {
        name: ActiveValue::Set(payload.name.clone()),
        email: ActiveValue::Set(payload.email.clone()),
        password: ActiveValue::Set("none".to_owned()),
        ..Default::default()
    };
    new_user.insert(db).await.unwrap();
    Json(UserDto {
        id: 7,
        name: payload.name,
        email: payload.email,
    })
}
