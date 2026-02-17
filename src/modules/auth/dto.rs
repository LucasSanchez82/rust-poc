

use crate::modules::user::dto::UserDto;

pub struct AuthSession {
    pub user: UserDto,
    pub session_token: Box<str>,
}
