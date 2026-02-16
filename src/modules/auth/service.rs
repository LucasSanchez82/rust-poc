use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sea_orm::DatabaseConnection;
use tracing::error;

use crate::modules::errors::ServiceError;
use crate::modules::session::dto::SessionTokenDTO;
use crate::modules::session::service::SessionService;
use crate::modules::types::ServiceResult;
use crate::modules::user::payload::LoginPayload;
use crate::modules::user::service::UserService;

pub struct AuthService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AuthService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn login(&self, payload: LoginPayload) -> ServiceResult<SessionTokenDTO> {
        let user_svc = UserService::new(self.db);
        let user = user_svc.get_per_email_model(&payload.email).await?;

        let password = payload.password;
        let password_hash = user.password;
        let email = payload.email;

        tokio::task::spawn_blocking(move || {
            let hash = PasswordHash::new(&password_hash).map_err(|err| {
                let err_msg = format!(
                    "Error, the hash of the user with email: '{}' is not valid",
                    email
                );
                error!("{}", err_msg);
                ServiceError::internal(err_msg).with_details(err.to_string())
            })?;

            Argon2::default()
                .verify_password(password.as_bytes(), &hash)
                .map_err(|_| ServiceError::unauthorized("Bad password"))
        })
        .await
        .map_err(|_| ServiceError::internal("Password verification task failed"))??;

        let session_svc = SessionService::new(self.db);
        session_svc.create(user.id).await
    }
}
