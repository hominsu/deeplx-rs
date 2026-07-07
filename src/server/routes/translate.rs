use super::AppState;
use crate::server::pkgs::{Error, Json};

use axum::{
    extract::{Form, State},
    http::{HeaderMap, header},
    response::Response,
};
use deeplx::Auth;
use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct PayloadFree {
    pub text: String,
    pub source_lang: String,
    pub target_lang: String,
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

    state
        .translate_uc
        .translate(&text, &source_lang, &target_lang, Auth::Anonymous)
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
    let token = bearer_token(&headers)
        .or_else(|| legacy_dl_session(&headers))
        .ok_or(Error::DeepLSessionMissing)?;

    state
        .translate_uc
        .translate(
            &text,
            &source_lang,
            &target_lang,
            Auth::Bearer(SecretString::from(token)),
        )
        .await
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

fn bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn legacy_dl_session(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                let cookie = cookie.trim();
                cookie
                    .strip_prefix("dl_session=")
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned)
            })
        })
}
