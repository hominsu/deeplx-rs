use crate::server::{biz::translate::TranslateUsecase, conf::Config, pkgs::Error};

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use std::{
    future::Future,
    sync::{Arc, RwLock},
};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub translate_uc: Arc<TranslateUsecase>,
    pub config: Arc<RwLock<Config>>,
}

impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    fn from_request_parts(
        _parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        async move { Ok(Self::from_ref(state)) }
    }
}
