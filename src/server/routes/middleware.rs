use super::AppState;
use crate::server::pkgs::Error;

use axum::{
    extract::{FromRef, FromRequestParts, Query},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

#[derive(serde::Deserialize)]
struct QueryParams {
    key: Option<String>,
}

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

        let auth = {
            let guard = state
                .config
                .read()
                .map_err(|_e| Error::InternalServerError)?;
            guard.auth.clone()
        };

        let query: Query<QueryParams> =
            Query::try_from_uri(&parts.uri).map_err(|_| Error::InternalServerError)?;

        let bearer = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(TypedHeader(Authorization(b))) => Some(b.token().to_owned()),
            Err(_) => None,
        };

        let token = query.key.as_deref().or_else(|| bearer.as_deref());

        if token != Some(auth.as_str()) {
            return Err(Error::InvalidAccessToken);
        }

        Ok(Self {})
    }
}
