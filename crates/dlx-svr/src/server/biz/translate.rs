use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use dlx::{Auth, TranslateResponse};
use serde::Serialize;

use crate::server::pkgs::{Error, Json};

pub trait TranslateRepo: Send + Sync {
    fn translate<'a>(
        &'a self,
        text: &'a str,
        source_lang: &'a str,
        target_lang: &'a str,
        auth: Auth,
    ) -> Pin<Box<dyn Future<Output = Result<TranslateResponse, Error>> + Send + 'a>>;
}

pub struct TranslateUsecase {
    repo: Arc<dyn TranslateRepo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TranslateResult {
    pub code: u16,
    pub id: i64,
    pub data: String,
    pub alternatives: Vec<String>,
    pub source_lang: String,
    pub target_lang: String,
    pub method: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Translation {
    pub detected_source_language: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TranslateResultOfficial {
    pub translations: Vec<Translation>,
}

impl TranslateUsecase {
    pub fn new(repo: Arc<dyn TranslateRepo>) -> Self {
        Self { repo }
    }

    pub async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        auth: Auth,
    ) -> Result<Response, Error> {
        let method = match &auth {
            Auth::Anonymous => "Free",
            Auth::Bearer(_) => "Pro",
        }
        .to_string();

        let res = self
            .repo
            .translate(text, source_lang, target_lang, auth)
            .await?;
        let first = res
            .translations
            .first()
            .ok_or(Error::Dlx(dlx::Error::MissingTranslation))?;

        Ok(Json(TranslateResult {
            code: StatusCode::OK.as_u16(),
            id: request_id(),
            data: first.text.clone(),
            alternatives: Vec::new(),
            source_lang: res
                .source_lang
                .unwrap_or_else(|| first.detected_source_language.clone()),
            target_lang: res.target_lang,
            method,
        })
        .with_status_code(StatusCode::OK)
        .into_response())
    }

    pub async fn translate_official(
        &self,
        text: &str,
        target_lang: &str,
    ) -> Result<Response, Error> {
        let res = self
            .repo
            .translate(text, "auto", target_lang, Auth::Anonymous)
            .await?;

        Ok(Json(TranslateResultOfficial {
            translations: res
                .translations
                .into_iter()
                .map(|translation| Translation {
                    detected_source_language: translation.detected_source_language,
                    text: translation.text,
                })
                .collect(),
        })
        .with_status_code(StatusCode::OK)
        .into_response())
    }
}

fn request_id() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or_default()
}
