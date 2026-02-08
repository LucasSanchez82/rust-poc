use serde::Serialize;

use crate::modules::models::entities::session::ActiveModel as SessionActiveModel;
use crate::modules::models::entities::session::Model as SessionModel;

#[derive(Serialize)]
pub struct SessionDTO {
    pub token: String,
}

impl From<SessionActiveModel> for SessionDTO {
    fn from(session: SessionActiveModel) -> Self {
        let token = session.token.try_as_ref().unwrap().to_string();
        Self { token }
    }
}

impl From<SessionModel> for SessionDTO {
    fn from(session: SessionModel) -> Self {
        let token = session.token.to_string();
        Self { token }
    }
}
