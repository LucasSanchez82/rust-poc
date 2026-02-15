use serde::Serialize;

use crate::modules::models::entities::session::Model as SessionModel;
use crate::modules::models::entities::user::Model as UserModel;
use crate::modules::user::dto::UserDto;

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

#[derive(Serialize)]
pub struct SessionUserDTO {
    pub token: String,
    pub user: Option<UserDto>,
}

impl From<(SessionModel, Option<UserModel>)> for SessionUserDTO {
    fn from((session, user): (SessionModel, Option<UserModel>)) -> Self {
        Self {
            token: session.token.to_string(),
            user: user.map(UserDto::from),
        }
    }
}
