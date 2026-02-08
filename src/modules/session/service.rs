use crate::modules::models::entities::session::ActiveModel as SessionActiveModel;

use crate::modules::errors::ServiceError;
use crate::modules::session::dto::SessionDTO;
use crate::modules::types::ServiceResult;
use chrono::{Duration, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use tracing::error;
use uuid::Uuid;

pub struct SessionService<'a> {
    pub db: &'a DatabaseConnection,
    pub token: Uuid,
    pub user_id: i32,
    pub created_at: DateTimeWithTimeZone,
    pub expire_at: DateTimeWithTimeZone,
}

impl<'a> SessionService<'a> {
    pub fn new_with_duration_hours(db: &'a DatabaseConnection, user_id: i32, hours: i64) -> Self {
        let token = Uuid::new_v4();
        let created_at = Utc::now();
        let expire_at = created_at + Duration::hours(hours);
        Self {
            db,
            token,
            user_id,
            created_at: created_at.into(),
            expire_at: expire_at.into(),
        }
    }

    pub fn new(db: &'a DatabaseConnection, user_id: i32) -> Self {
        Self::new_with_duration_hours(db, user_id, 720)
    }

    pub async fn create(&self) -> ServiceResult<SessionDTO> {
        let session_entity: SessionActiveModel = self.into();
        let session = session_entity.insert(self.db).await;
        match session {
            Err(err) => {
                let msg = "Internal error during the creation of the session";
                error!("message: {}\ndetails: {}", msg, err);

                Err(ServiceError::internal(msg).with_details(err.to_string()))
            }
            Ok(session_active_model) => Ok(session_active_model.into()),
        }
    }
}

impl From<&SessionService<'_>> for SessionActiveModel {
    fn from(session: &SessionService) -> SessionActiveModel {
        Self {
            created_at: Set(session.created_at),
            expire_at: Set(session.expire_at),
            token: Set(session.token),
            user_id: Set(session.user_id),
        }
    }
}
