use serde::Serialize;

use crate::modules::models::entities::user::Model as UserModel;

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<UserModel> for UserDto {
    fn from(user: UserModel) -> Self {
        UserDto {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
