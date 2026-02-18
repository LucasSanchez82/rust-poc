use axum::{
    extract::{FromRequest, Json},
    http::{Request, StatusCode},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::modules::responses::ApiError;

pub struct ExtractValidated<T>(pub T);

impl<T, S> FromRequest<S> for ExtractValidated<T>
where
    T: Validate + DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request<axum::body::Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| ApiError::new(StatusCode::BAD_REQUEST, err.to_string(), None))?;

        payload.validate()?;

        Ok(ExtractValidated(payload))
    }
}
