use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use axum::{
    Extension,
    body::{Body, Bytes},
    extract::{Request, State},
    http::{self, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use tracing::debug;

use crate::modules::{
    responses::ApiError, session::service::SessionService, states::AppState, user::dto::UserDto,
};

use axum::{
    Json,
    extract::FromRequestParts,
    http::{
        header::{HeaderValue, USER_AGENT},
        request::Parts,
    },
};
use serde::{Deserialize, Serialize};

pub struct AuthState {
    user: Option<UserDto>,
}

pub struct ExtractAuthInfos(UserDto);

impl FromRequestParts<AppState> for ExtractAuthInfos {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        app_state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(authorization) = parts.headers.get(AUTHORIZATION) {
            let dbg = authorization.to_str();
            match dbg {
                Err(_) => Err((
                    StatusCode::BAD_REQUEST,
                    "`authorization` header is malformated".to_owned(),
                )),
                Ok(auth_str) => {
                    let token = auth_str.replace("Bearer ", "");

                    let session_service = SessionService::new(&app_state.connection);
                    let session = session_service.get_with_user(token.as_str()).await?;
                    session.user
                        .ok_or((StatusCode::BAD_REQUEST, "The related session has no linked user".to_owned()))
                        .map(ExtractAuthInfos)
                }
            }
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                "`authorization` header is missing".to_owned(),
            ))
        }
    }
}

pub async fn test_handler(ExtractAuthInfos(user): ExtractAuthInfos) -> Json<UserDto> {
    Json(user)
}
