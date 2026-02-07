use serde::Serialize;

use crate::modules::models::entities::user::Model as UserModel;

#[derive(Serialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// TODO: TEST IF THAT POSSIBLE TO USE &str INSTEAD OF STRING using to_owned()
impl From<&UserModel> for UserDto {
    fn from(user: &UserModel) -> Self {
        UserDto {
            id: user.id,
            name: user.name.to_owned(),
            email: user.email.to_owned(),
        }
    }
}

impl From<UserModel> for UserDto {
    fn from(user: UserModel) -> Self {
        UserDto::from(&user)
    }
}
