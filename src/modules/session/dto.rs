use serde::Serialize;

use crate::modules::models::entities::session::Model as SessionModel;

#[derive(Serialize)]
pub struct SessionDTO {
    pub token: String,
}

impl From<SessionModel> for SessionDTO {
    fn from(session: SessionModel) -> Self {
        let token = session.token.to_string();
        Self { token }
    }
}
