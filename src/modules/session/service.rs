use std::str::FromStr;

use crate::modules::models::entities::session::ActiveModel as SessionActiveModel;
use crate::modules::models::entities::session::Column as SessionColumn;
use crate::modules::models::entities::session::Entity as SessionEntity;

use crate::modules::models::entities::user::Entity as UserEntity;

use crate::modules::errors::ServiceError;
use crate::modules::session::dto::SessionTokenDTO;
use crate::modules::session::dto::SessionUserDTO;
use crate::modules::types::ServiceResult;
use chrono::{Duration, Utc};
use migration::Expr;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use tower_http::trace;
use tracing::error;
use tracing::trace;
use uuid::Uuid;

pub struct SessionService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SessionService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, user_id: i32) -> ServiceResult<SessionTokenDTO> {
        let token = Uuid::new_v4();
        let created_at = Utc::now();
        let expire_at = created_at + Duration::hours(720);

        let session_entity = SessionActiveModel {
            token: Set(token),
            user_id: Set(user_id),
            revokated_at: NotSet,
            created_at: Set(created_at.into()),
            expire_at: Set(expire_at.into()),
        };

        session_entity
            .insert(self.db)
            .await
            .map(SessionTokenDTO::from)
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
                            trace!("session: {:#?}", session_user_dto);
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

    pub async fn revoke_token(&self, id: String) -> ServiceResult<SessionTokenDTO> {
        let token_uuid = Uuid::from_str(id.as_str())
            .map_err(|_error| ServiceError::bad_request("The given token is malformated"))?;

        let result = SessionEntity::update_many()
            .col_expr(
                SessionColumn::RevokatedAt,
                Expr::value(Utc::now().fixed_offset()),
            )
            .filter(SessionColumn::Token.eq(token_uuid))
            .exec(self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(ServiceError::bad_request("No session founded"));
        }

        Ok(SessionTokenDTO { token: id })
    }
}
