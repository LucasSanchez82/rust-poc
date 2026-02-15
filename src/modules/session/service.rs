use crate::modules::models::entities::session::ActiveModel as SessionActiveModel;
use crate::modules::models::entities::session::Entity as SessionEntity;

use crate::modules::models::entities::user::Entity as UserEntity;

use crate::modules::errors::ServiceError;
use crate::modules::session::dto::SessionDTO;
use crate::modules::session::dto::SessionUserDTO;
use crate::modules::types::ServiceResult;
use chrono::{Duration, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use tracing::error;
use uuid::Uuid;

pub struct SessionService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SessionService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, user_id: i32) -> ServiceResult<SessionDTO> {
        let token = Uuid::new_v4();
        let created_at = Utc::now();
        let expire_at = created_at + Duration::hours(720);

        let session_entity = SessionActiveModel {
            token: Set(token),
            user_id: Set(user_id),
            created_at: Set(created_at.into()),
            expire_at: Set(expire_at.into()),
        };

        session_entity
            .insert(self.db)
            .await
            .map(SessionDTO::from)
            .map_err(|err| {
                let msg = "Internal error during the creation of the session";
                error!("message: {}\ndetails: {}", msg, err);
                ServiceError::internal(msg).with_details(err.to_string())
            })
    }

    pub async fn get_with_user(&self, token: &str) -> ServiceResult<SessionUserDTO> {
        let token = Uuid::parse_str(token);
        match token {
            Err(err) => Err(
                ServiceError::bad_request("The provided auth token is malformated")
                    .with_details(err.to_string()),
            ),
            Ok(token) => {
                let session_user = SessionEntity::find_by_id(token)
                    .find_also_related(UserEntity)
                    .one(self.db)
                    .await;

                match session_user {
                    Ok(session_user) => match session_user {
                        Some(session_user) => {
                            let session_user_dto = SessionUserDTO::from(session_user);
                            Ok(session_user_dto)
                        }
                        None => Err(ServiceError::internal(
                            "Internal error during session recovery",
                        )),
                    },
                    Err(err) => Err(ServiceError::internal(
                        "Internal error during session recovery",
                    )
                    .with_details(err.to_string())),
                }
            }
        }
    }
}
