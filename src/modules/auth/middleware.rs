use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use axum::{
    Extension,
    body::{Body, Bytes},
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use tracing::debug;

use crate::modules::user::dto::UserDto;

pub struct AuthState {
    user: Option<UserDto>,
}

pub async fn auth_handler(state: Extension<Arc<AuthState>>) -> String {
    return match &state.user {
        None => "No user".to_owned(),
        Some(user) => format!("user_id: {}", user.id),
    };
}

pub async fn print_request_response(
    State(counter): State<Arc<AtomicU64>>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let count = counter.fetch_add(1, Ordering::Relaxed);
    tracing::debug!("request count: {count}");

    let (parts, body) = req.into_parts();
    debug!("headers: {:#?}", parts.headers);
    let authorization_header = parts.headers.get(http::header::AUTHORIZATION);

    match authorization_header {
        None => {
            debug!("There is no authorization header...");
        }
        Some(header) => {
            debug!(
                "There is header: {}",
                header.to_str().unwrap_or("no header")
            )
        }
    }

    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
