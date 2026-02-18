use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};

use crate::modules::auth::extractor::ExtractAuthInfos;
use crate::modules::auth::service::AuthService;
use crate::modules::responses::ApiError;
use crate::modules::session::dto::SessionTokenDTO;
use crate::modules::session::service::SessionService;
use crate::modules::states::AppState;
use crate::modules::types::ApiResponse;
use crate::modules::user::dto::UserDto;
use crate::modules::user::payload::LoginPayload;
use crate::utils::extractor::ExtractValidated;

pub async fn handle_me(ExtractAuthInfos(auth_session): ExtractAuthInfos) -> Json<UserDto> {
    Json(auth_session.user)
}

pub async fn handle_login(
    State(state): State<AppState>,
    ExtractValidated(payload): ExtractValidated<LoginPayload>,
) -> ApiResponse<SessionTokenDTO> {
    let auth_svc = AuthService::new(&state.connection);
    auth_svc
        .login(payload)
        .await
        .map(Json)
        .map_err(ApiError::from)
}

pub async fn handle_logout(
    State(app_state): State<AppState>,
    ExtractAuthInfos(auth_session): ExtractAuthInfos,
) -> ApiResponse<SessionTokenDTO> {
    let session_service = SessionService::new(&app_state.connection);
    let session_token_dto = session_service
        .revoke_token(auth_session.session_token.to_string())
        .await?;
    Ok(Json(SessionTokenDTO {
        token: session_token_dto.token,
    }))
}

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(handle_login))
        .route("/logout", post(handle_logout))
        .route("/me", post(handle_me))
}
