use crate::server::pkgs::{Error, Json};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use deeplx::DeepLXTranslationResult;
use serde::Serialize;
use std::{future::Future, pin::Pin, sync::Arc};

pub trait TranslateRepo: Send + Sync {
    fn translate<'a>(
        &'a self,
        text: &'a str,
        source_lang: &'a str,
        target_lang: &'a str,
        tag_handling: Option<&'a str>,
        dl_session: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Result<DeepLXTranslationResult, Error>> + Send + 'a>>;
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
pub struct TranslateResultUnknown {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
        tag_handling: Option<&str>,
        dl_session: Option<&str>,
    ) -> Result<Response, Error> {
        let res = self
            .repo
            .translate(text, source_lang, target_lang, tag_handling, dl_session)
            .await?;

        match res.code {
            200 => Ok(Json(TranslateResult {
                code: res.code as u16,
                id: res.id,
                data: res.data,
                alternatives: res.alternatives,
                source_lang: res.source_lang,
                target_lang: res.target_lang,
                method: res.method,
            })
            .with_status_code(StatusCode::OK)
            .into_response()),
            _ => Ok(Json(TranslateResultUnknown {
                code: res.code as u16,
                message: res.message,
            })
            .with_status_code(StatusCode::from_u16(res.code as u16).unwrap())
            .into_response()),
        }
    }

    pub async fn translate_official(
        &self,
        text: &str,
        target_lang: &str,
    ) -> Result<Response, Error> {
        let res = self
            .repo
            .translate(text, "auto", target_lang, None, None)
            .await?;

        match res.code {
            200 => Ok(Json(TranslateResultOfficial {
                translations: vec![Translation {
                    detected_source_language: res.source_lang,
                    text: res.data,
                }],
            })
            .with_status_code(StatusCode::OK)
            .into_response()),
            _ => Ok(Json(TranslateResultUnknown {
                code: res.code as u16,
                message: res.message,
            })
            .with_status_code(StatusCode::from_u16(res.code as u16).unwrap())
            .into_response()),
        }
    }
}
