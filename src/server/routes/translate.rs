use super::AppState;
use crate::server::pkgs::{Error, Json};

use axum::{
    extract::{Form, State},
    http::{header, HeaderMap},
    response::Response,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct PayloadFree {
    pub text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub tag_handling: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct PayloadForm {
    pub text: String,
    pub target_lang: String,
}

pub async fn translate_free(
    State(state): State<AppState>,
    Json(payload): Json<PayloadFree>,
) -> Result<Response, Error> {
    let text = payload.text;
    let source_lang = payload.source_lang;
    let target_lang = payload.target_lang;
    let tag_handling = payload.tag_handling.as_deref();

    if tag_handling.is_some_and(|tag_handling| !matches!(tag_handling, "html" | "xml")) {
        return Err(Error::InvalidTagHandling);
    }

    state
        .translate_uc
        .translate(&text, &source_lang, &target_lang, tag_handling, None)
        .await
}

pub async fn translate_pro(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<PayloadFree>,
) -> Result<Response, Error> {
    let text = payload.text;
    let source_lang = payload.source_lang;
    let target_lang = payload.target_lang;
    let tag_handling = payload.tag_handling.as_deref();
    let dl_session = headers
        .get(header::COOKIE)
        .map(|c| c.to_str().unwrap().replace("dl_session=", ""));

    if tag_handling.is_some_and(|tag_handling| !matches!(tag_handling, "html" | "xml")) {
        return Err(Error::InvalidTagHandling);
    }

    match dl_session {
        None => Err(Error::DeepLSessionMissing),
        Some(dl_session) => match dl_session.contains(".") {
            true => Err(Error::DeepLUnauthorized),
            false => {
                state
                    .translate_uc
                    .translate(
                        &text,
                        &source_lang,
                        &target_lang,
                        tag_handling,
                        Some(dl_session.as_str()),
                    )
                    .await
            }
        },
    }
}

pub async fn translate_official(
    State(state): State<AppState>,
    Form(payload): Form<PayloadForm>,
) -> Result<Response, Error> {
    let text = payload.text;
    let target_lang = payload.target_lang;

    state
        .translate_uc
        .translate_official(&text, &target_lang)
        .await
}
