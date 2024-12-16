use super::AppState;
use crate::server::pkgs::Error;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

pub struct RequireAuth {}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = parts
            .extract_with_state::<AppState, _>(state)
            .await
            .map_err(|_e| Error::InternalServerError)?;

        let auth = state
            .config
            .read()
            .map_err(|_e| Error::InternalServerError)?
            .auth
            .clone();

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::InvalidAccessToken)?;

        match auth.eq(bearer.token()) {
            true => Ok(Self {}),
            false => Err(Error::InvalidAccessToken),
        }
    }
}
