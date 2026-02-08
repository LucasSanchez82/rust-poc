use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use rand::{Rng, distr::Alphanumeric};
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, ActiveValue};
use tracing::info;

use crate::modules::models::entities::user::ActiveModel as UserActiveModel;
use crate::modules::models::entities::user::Entity as UserEntity;
use crate::modules::types::ServiceResult;
use crate::modules::user::dto::UserDto;
use crate::modules::user::payload::CreateUser;
use crate::modules::{errors::ServiceError, user::payload::LoginPayload};

pub struct UserService<'a> {
    db: Arc<DatabaseConnection>,
    argon2: Argon2<'a>,
}

#[allow(async_fn_in_trait)]
pub trait Service<T, P> {
    fn new(db: Arc<DatabaseConnection>) -> Self;
    fn db(&self) -> &DatabaseConnection;
    async fn create(&self, payload: P) -> ServiceResult<T>;
    async fn get_all(&self) -> ServiceResult<Box<[T]>>;
    async fn update(&self, id: i32) -> ServiceResult<T>;
    async fn delete(&self, id: i32) -> ServiceResult<T>;
    async fn get_one(&self, id: i32) -> ServiceResult<T>;
}

impl<'a> UserService<'a> {
    fn gen_token() -> String {
        rand::rng()
            .sample_iter(Alphanumeric)
            .take(35)
            .map(char::from)
            .collect()
    }

    fn argon2(&self) -> &Argon2<'a> {
        &self.argon2
    }

    fn login(&self, payload: LoginPayload) {
        todo!()
    }

    async fn get_per_email(&self, id: i32) -> ServiceResult<UserDto> {
        let user = UserEntity::find_by_email(id).one(self.db()).await;

        match user {
            Ok(user) => {
                if let Some(user) = user {
                    Ok(UserDto::from(user))
                } else {
                    Err(ServiceError::not_found(format!("The user with id {}", id)))
                }
            }
            Err(err) => {
                Err(ServiceError::internal("Failed to fetch users").with_details(err.to_string()))
            }
        }
    }
}

impl<'a> Service<UserDto, CreateUser> for UserService<'a> {
    fn new(db: Arc<DatabaseConnection>) -> Self {
        let argon2 = Argon2::default();

        Self { db, argon2 }
    }

    fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    async fn create(&self, payload: CreateUser) -> ServiceResult<UserDto> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2()
            .hash_password(payload.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let new_user = UserActiveModel {
            name: ActiveValue::Set(payload.name),
            email: ActiveValue::Set(payload.email),
            password: ActiveValue::Set(password_hash),
            ..Default::default()
        };

        let created_user = new_user.insert(self.db()).await.map_err(|e| {
            ServiceError::internal("Failed to create user").with_details(e.to_string())
        })?;

        Ok(UserDto {
            id: created_user.id,
            name: created_user.name,
            email: created_user.email,
        })
    }

    async fn delete(&self, id: i32) -> ServiceResult<UserDto> {
        let deleted_user = UserEntity::delete_by_id(id)
            .exec_with_returning(self.db())
            .await
            .map_err(|e| {
                ServiceError::internal(format!("Error during delete of user with id: {}", id))
                    .with_details(e.to_string())
            })?;

        deleted_user
            .map(UserDto::from)
            .ok_or_else(|| ServiceError::not_found(format!("User with id {} not found", id)))
    }

    async fn get_all(&self) -> ServiceResult<Box<[UserDto]>> {
        let users = UserEntity::find().all(self.db()).await.map_err(|e| {
            ServiceError::internal("Failed to fetch users").with_details(e.to_string())
        })?;

        info!("{:#?}", users);

        Ok(users
            .iter()
            .map(UserDto::from)
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    async fn get_one(&self, id: i32) -> ServiceResult<UserDto> {
        let user = UserEntity::find_by_id(id).one(self.db()).await;

        match user {
            Ok(user) => {
                if let Some(user) = user {
                    Ok(UserDto::from(user))
                } else {
                    Err(ServiceError::not_found(format!("The user with id {}", id)))
                }
            }
            Err(err) => {
                Err(ServiceError::internal("Failed to fetch users").with_details(err.to_string()))
            }
        }
    }

    async fn update(&self, _id: i32) -> ServiceResult<UserDto> {
        todo!()
    }
}

pub struct Claim {
    pub exp: u64,
    pub iat: u64,
    pub email: String,
}
