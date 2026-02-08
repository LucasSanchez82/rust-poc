use axum::Json;
use axum::extract::State;
use validator::Validate;

use crate::modules::auth::service::AuthService;
use crate::modules::errors::ServiceError;
use crate::modules::responses::ApiError;
use crate::modules::session::dto::SessionDTO;
use crate::modules::states::AppState;
use crate::modules::types::ApiResponse;
use crate::modules::user::payload::LoginPayload;

pub async fn handle_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> ApiResponse<SessionDTO> {
    payload.validate().map_err(|e| {
        ApiError::from(
            ServiceError::new(axum::http::StatusCode::BAD_REQUEST, "Validation error")
                .with_details(e.to_string()),
        )
    })?;

    let auth_svc = AuthService::new(&state.connection);
    auth_svc
        .login(payload)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
