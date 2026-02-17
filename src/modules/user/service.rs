use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::extract::State;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, ActiveValue};
use sea_orm::{DatabaseConnection, PaginatorTrait};

use crate::modules::models::entities::user::Entity as UserEntity;
use crate::modules::models::entities::user::Model as UserModel;
use crate::modules::{models::entities::user::ActiveModel as UserActiveModel, states::AppState};

use crate::modules::errors::ServiceError;
use crate::modules::types::ServiceResult;
use crate::modules::user::dto::UserDto;
use crate::modules::user::payload::CreateUser;

pub struct UserService<'a> {
    db: &'a DatabaseConnection,
    argon2: Argon2<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self {
            db,
            argon2: Argon2::default(),
        }
    }

    pub async fn create(&self, payload: CreateUser) -> ServiceResult<UserDto> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(payload.password.as_bytes(), &salt)
            .map_err(|err| ServiceError::internal("internal error").with_details(err.to_string()))?
            .to_string();

        let new_user = UserActiveModel {
            name: ActiveValue::Set(payload.name),
            email: ActiveValue::Set(payload.email),
            password: ActiveValue::Set(password_hash),
            ..Default::default()
        };

        let created_user = new_user.insert(self.db).await.map_err(|e| {
            ServiceError::internal("Failed to create user").with_details(e.to_string())
        })?;

        Ok(UserDto {
            id: created_user.id,
            name: created_user.name,
            email: created_user.email,
        })
    }

    pub async fn create_initial_root_user(&self, payload: CreateUser) -> ServiceResult<UserDto> {
        let users_count = self.get_activated_users_count().await?;

        if users_count < 1 {
            Err(ServiceError::not_found("No longer available"))
        } else {
            Ok(self.create(payload).await?)
        }
    }

    pub async fn get_activated_users_count(&self) -> ServiceResult<u64> {
        UserEntity::find().count(self.db).await.map(Ok)?
    }

    pub async fn get_all(&self) -> ServiceResult<Vec<UserDto>> {
        let users = UserEntity::find().all(self.db).await.map_err(|e| {
            ServiceError::internal("Failed to fetch users").with_details(e.to_string())
        })?;

        Ok(users.into_iter().map(UserDto::from).collect())
    }

    pub async fn get_one(&self, id: i32) -> ServiceResult<UserDto> {
        let user = UserEntity::find_by_id(id)
            .one(self.db)
            .await
            .map_err(|err| {
                ServiceError::internal("Failed to fetch users").with_details(err.to_string())
            })?;

        user.map(UserDto::from)
            .ok_or_else(|| ServiceError::not_found(format!("The user with id {}", id)))
    }

    pub async fn delete(&self, id: i32) -> ServiceResult<UserDto> {
        let deleted_user = UserEntity::delete_by_id(id)
            .exec_with_returning(self.db)
            .await
            .map_err(|e| {
                ServiceError::internal(format!("Error during delete of user with id: {}", id))
                    .with_details(e.to_string())
            })?;

        deleted_user
            .map(UserDto::from)
            .ok_or_else(|| ServiceError::not_found(format!("User with id {} not found", id)))
    }

    pub async fn get_per_email_model(&self, email: &str) -> ServiceResult<UserModel> {
        let user = UserEntity::find_by_email(email)
            .one(self.db)
            .await
            .map_err(|err| {
                ServiceError::internal("Failed to fetch users").with_details(err.to_string())
            })?;

        user.ok_or_else(|| {
            ServiceError::not_found(format!("The user with email {} not found", email))
        })
    }
}
