use serde::{Deserialize, Deserializer};
use validator::Validate;

fn deserialize_lowercase<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    String::deserialize(deserializer).map(|s| s.to_lowercase())
}

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[serde(deserialize_with = "deserialize_lowercase")]
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[serde(deserialize_with = "deserialize_lowercase")]
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}
