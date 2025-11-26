use super::AppState;
use crate::server::pkgs::Error;

use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts, Query},
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

#[derive(serde::Deserialize)]
struct QueryParams {
    key: Option<String>,
    token: Option<String>,
}

pub struct RequireAuth {}

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
            .map_err(|_e| Error::InternalServer)?;

        let auth_option = {
            let guard = state.config.read().map_err(|_e| Error::InternalServer)?;
            guard.auth.clone()
        };

        let auth = match auth_option {
            None => return Ok(Self {}),
            Some(ref s) if s.is_empty() => return Ok(Self {}),
            Some(s) => s,
        };

        let query: Query<QueryParams> =
            Query::try_from_uri(&parts.uri).map_err(|_| Error::InternalServer)?;

        let bearer = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(TypedHeader(Authorization(b))) => Some(b.token().to_owned()),
            Err(_) => None,
        };

        // Priority: bearer > token > key
        let token = [
            bearer.as_deref(),
            query.token.as_deref(),
            query.key.as_deref(),
        ]
        .into_iter()
        .find_map(|t| t);

        if token != Some(auth.as_str()) {
            return Err(Error::InvalidAccessToken);
        }

        Ok(Self {})
    }
}
