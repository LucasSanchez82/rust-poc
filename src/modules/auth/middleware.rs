
use axum::http::{StatusCode, header::AUTHORIZATION};

use crate::modules::{
    session::service::SessionService, states::AppState, user::dto::UserDto,
};

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

pub struct ExtractAuthInfos(pub UserDto);

impl FromRequestParts<AppState> for ExtractAuthInfos {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        app_state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let authorization = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or((StatusCode::UNAUTHORIZED, "`authorization` header is missing".to_owned()))?;

        let auth_str = authorization.to_str().map_err(|_| {
            (StatusCode::UNAUTHORIZED, "`authorization` header is malformed".to_owned())
        })?;

        let token = auth_str
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization scheme".to_owned()))?;

        let session_service = SessionService::new(&app_state.connection);
        let session = session_service.get_with_user(token).await?;

        session
            .user
            .ok_or((StatusCode::UNAUTHORIZED, "The related session has no linked user".to_owned()))
            .map(ExtractAuthInfos)
    }
}
