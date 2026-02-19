use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct DockerCreatePayload {
    pub dockerfile: String,
    pub tag: String,
    pub rm: bool,
}

#[derive(Validate, Deserialize)]
pub struct DockerMariadbPayload {
    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "root_password cannot be empty"))]
    pub root_password: String,
    pub database: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub port: Option<u16>,
}
