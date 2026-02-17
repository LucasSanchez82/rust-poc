use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::modules::{session::dto::SessionTokenDTO, user::dto::UserDto};

pub struct AuthSession {
    pub user: UserDto,
    pub session_token: Box<str>,
}
