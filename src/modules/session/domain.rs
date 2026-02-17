use chrono::{DateTime, Utc};
use tracing::trace;

use crate::modules::models::entities::session::Model as SessionModel;
use crate::modules::models::entities::user::Model as UserModel;
use crate::modules::user::dto::UserDto;

#[derive(Debug)]
pub struct SessionWithUser {
    pub token: String,
    pub revoked_at: Option<DateTime<Utc>>,
    pub expire_at: DateTime<Utc>,
    pub user: Option<UserDto>,
}

impl From<(SessionModel, Option<UserModel>)> for SessionWithUser {
    fn from((session, user): (SessionModel, Option<UserModel>)) -> Self {
        Self {
            token: session.token.to_string(),
            revoked_at: session.revokated_at.map(|dt| dt.to_utc()),
            expire_at: session.expire_at.to_utc(),
            user: user.map(UserDto::from),
        }
    }
}

impl SessionWithUser {
    pub fn is_valid(&self) -> bool {
        trace!("Revoked at : {:#?}", self.revoked_at.is_some());
        trace!("Revoked is some : {:#?}", self.revoked_at);
        (Utc::now() < self.expire_at) && self.revoked_at.is_none()
    }
}
