pub mod label;
pub mod todo;
pub mod user;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    BoxError, Json,
};
use serde::{
    Deserialize,
    de::DeserializeOwned,
};
use validator::Validate;

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[derive(Deserialize)]
pub struct UserIdQuery {
    pub user_id: i32,
}

#[async_trait]
impl <T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await.map_err(|rejection| {
            let message = format!("Json parse error: [{}]", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}