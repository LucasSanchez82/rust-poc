use axum::http::{StatusCode, header::AUTHORIZATION};

use crate::modules::{
    responses::ApiError, session::service::SessionService, states::AppState, user::dto::UserDto,
};

use axum::{extract::FromRequestParts, http::request::Parts};

pub struct ExtractAuthInfos(pub UserDto);

impl FromRequestParts<AppState> for ExtractAuthInfos {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        app_state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let authorization = parts.headers.get(AUTHORIZATION).ok_or(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "`authorization` header is missing".to_owned(),
            None,
        ))?;

        let auth_str = authorization.to_str().map_err(|_| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "`authorization` header is malformed".to_owned(),
                None,
            )
        })?;

        let token = auth_str.strip_prefix("Bearer ").ok_or(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid authorization scheme".to_owned(),
            None,
        ))?;

        let session_service = SessionService::new(&app_state.connection);
        let session = session_service.get_with_user(token).await?;

        if !session.is_valid() {
            return Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "Session is not valid".to_owned(),
                None,
            ));
        }

        session
            .user
            .ok_or(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "The related session has no linked user".to_owned(),
                None,
            ))
            .map(ExtractAuthInfos)
    }
}
