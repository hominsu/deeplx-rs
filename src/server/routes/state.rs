use crate::server::{biz::translate::TranslateUsecase, conf::Config, pkgs::Error};

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use std::sync::{Arc, RwLock};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub translate_uc: Arc<TranslateUsecase>,
    pub config: Arc<RwLock<Config>>,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
