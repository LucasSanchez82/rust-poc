use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Result;
use axum::routing::{delete, get, post};
use axum::{Json, Router, extract};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::modules::models::entities::user::ActiveModel as UserActiveModel;
use crate::modules::models::entities::user::Column as UserColumn;
use crate::modules::models::entities::user::Entity as UserEntity;
use crate::modules::models::entities::user::Model as UserModel;
use crate::modules::responses::ApiError;
use sea_orm::EntityTrait;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct DeleteUser {
    email: String,
}

#[derive(Serialize)]
struct UserDto {
    id: i32,
    name: String,
    email: String,
}

// impl From<UserActiveModel> FromUserActiveModel for UserDto {}
impl From<&UserModel> for UserDto {
    fn from(user: &UserModel) -> Self {
        UserDto {
            id: user.id.to_owned(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
        }
    }
}
pub fn user_router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", post(handle_create_user))
        .route("/", get(handle_get_users))
        .route("/", delete(handle_delete_user))
}

async fn handle_get_users(State(db): State<DatabaseConnection>) -> Json<Vec<UserDto>> {
    let users = UserEntity::find().all(&db).await.unwrap();
    info!("{:#?}", users);
    let users: Vec<UserDto> = users.iter().map(|user| UserDto::from(user)).collect();

    Json(users)
}

async fn handle_delete_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<DeleteUser>,
) -> Result<Json<Vec<UserDto>>, ApiError> {
    let deleted_user = UserEntity::delete_many()
        .filter(UserColumn::Email.eq(&payload.email))
        .exec_with_returning(&db)
        .await;

    return match deleted_user {
        Err(error) => {
            let error_message =
                format!("Error during delete of users with email: {}", payload.email);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                error_message,
                Some(error.to_string()),
            ))
        }
        Ok(deleted_user) => {
            let deleted_users_dto: Vec<UserDto> = deleted_user
                .iter()
                .map(|user| UserDto::from(user))
                .collect();
            return Ok(Json(deleted_users_dto));
        }
    };
}

async fn handle_create_user(
    State(db): State<DatabaseConnection>,
    extract::Json(payload): extract::Json<CreateUser>,
) -> Json<UserDto> {
    // use crate::modules::models::entities::post::Entity as Post;

    let new_user = UserActiveModel {
        name: ActiveValue::Set(payload.name.clone()),
        email: ActiveValue::Set(payload.email.clone()),
        password: ActiveValue::Set("none".to_owned()),
        ..Default::default()
    };
    new_user.insert(&db).await.unwrap();
    Json(UserDto {
        id: 7,
        name: payload.name,
        email: payload.email,
    })
}
