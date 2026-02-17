use chrono::{DateTime, Utc};
use serde::Serialize;
use tracing::trace;

use crate::modules::models::entities::session::Model as SessionModel;
use crate::modules::models::entities::user::Model as UserModel;
use crate::modules::user::dto::UserDto;

#[derive(Serialize)]
pub struct SessionTokenDTO {
    pub token: String,
}

impl From<SessionModel> for SessionTokenDTO {
    fn from(session: SessionModel) -> Self {
        let token = session.token.to_string();
        Self { token }
    }
}

#[derive(Debug, Serialize)]
pub struct SessionUserDTO {
    pub token: String,
    pub revoked_at: Option<DateTime<Utc>>,
    pub expire_at: DateTime<Utc>,
    pub user: Option<UserDto>,
}

impl From<(SessionModel, Option<UserModel>)> for SessionUserDTO {
    fn from((session, user): (SessionModel, Option<UserModel>)) -> Self {
        Self {
            token: session.token.to_string(),
            revoked_at: session.revokated_at.map(|dt| dt.to_utc()),
            expire_at: session.expire_at.to_utc(),
            user: user.map(UserDto::from),
        }
    }
}

impl SessionUserDTO {
    pub fn is_valid(&self) -> bool {
        trace!("Revoked at : {:#?}", self.revoked_at.is_some());
        trace!("Revoked is some : {:#?}", self.revoked_at);
        (Utc::now() < self.expire_at) && self.revoked_at.is_none()
    }
}

#[derive(Serialize)]
pub struct LogoutSuccessDTO {
    message: String,
}
