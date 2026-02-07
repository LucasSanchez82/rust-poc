use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use tracing::info;

use crate::modules::models::entities::user::Entity as UserEntity;
use crate::modules::responses::ApiError;
use crate::modules::types::ApiResponse;
use crate::modules::user::dto::UserDto;

pub struct UserService {
    db: Arc<DatabaseConnection>,
}

#[allow(async_fn_in_trait)]
pub trait Service<T> {
    fn new(db: Arc<DatabaseConnection>) -> Self;
    fn db(&self) -> &DatabaseConnection;
    async fn create(&self) -> T;
    async fn get_all(&self) -> Box<[T]>;
    async fn update(&self, id: i32) -> T;
    async fn delete(&self, id: i32) -> ApiResponse<T>;
    async fn get_one(&self, id: i32) -> T;
}

impl Service<UserDto> for UserService {
    fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
    fn db(&self) -> &DatabaseConnection {
        &self.db
    }
    async fn create(&self) -> UserDto {
        todo!();
    }
    async fn delete(&self, id: i32) -> ApiResponse<UserDto> {
        let db = self.db.as_ref();
        let deleted_user = UserEntity::delete_by_id(id).exec_with_returning(db).await;

        return match deleted_user {
            Err(error) => {
                let error_message = format!("Error during delete of users with id: {}", id);
                Err(ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    error_message,
                    Some(error.to_string()),
                ))
            }
            Ok(deleted_user) => {
                if let Some(deleted_user) = deleted_user {
                    return Ok(Json(UserDto::from(deleted_user)));
                } else {
                    return Err(ApiError::new(
                        StatusCode::NOT_FOUND,
                        format!("The requested user with id {} is not found", id),
                        None,
                    ));
                }
            }
        };
    }
    async fn get_all(&self) -> Box<[UserDto]> {
        let users = UserEntity::find().all(self.db()).await.unwrap();
        info!("{:#?}", users);
        let users: Box<[UserDto]> = users
            .iter()
            .map(|user| UserDto::from(user))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        users
    }
    async fn get_one(&self, _id: i32) -> UserDto {
        todo!();
    }
    async fn update(&self, _id: i32) -> UserDto {
        todo!();
    }
}
