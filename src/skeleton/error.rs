use std::time::Duration;

use reqwest::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("empty text")]
    EmptyText,

    #[error("text exceeds maximum length: {actual} characters, limit is {limit}")]
    TextTooLong { actual: usize, limit: usize },

    #[error("target_lang is required")]
    MissingTargetLang,

    #[error("target_lang cannot be auto")]
    AutoTargetLang,

    #[error("unsupported source language: {0}")]
    UnsupportedSourceLang(String),

    #[error("unsupported target language: {0}")]
    UnsupportedTargetLang(String),

    #[error("pro endpoint is required for bearer authentication")]
    MissingProEndpoint,

    #[error("missing translation in upstream response")]
    MissingTranslation,

    #[error("too many requests")]
    TooManyRequests,

    #[error("upstream returned HTTP {status}: {body}")]
    UpstreamStatus { status: StatusCode, body: String },

    #[error("upstream timeout after {0:?}")]
    Timeout(Duration),

    #[error(transparent)]
    Transport(#[from] reqwest::Error),

    #[error(transparent)]
    Impit(#[from] impit::errors::ImpitError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("invalid HTTP header")]
    InvalidHeader,

    #[error("invalid URL")]
    InvalidUrl,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::EmptyText => StatusCode::NOT_FOUND,
            Self::TextTooLong { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            Self::MissingTargetLang
            | Self::AutoTargetLang
            | Self::UnsupportedSourceLang(_)
            | Self::UnsupportedTargetLang(_)
            | Self::InvalidHeader
            | Self::InvalidUrl
            | Self::MissingProEndpoint => StatusCode::BAD_REQUEST,
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
            Self::UpstreamStatus { status, .. } => *status,
            Self::MissingTranslation
            | Self::Transport(_)
            | Self::Impit(_)
            | Self::Json(_)
            | Self::Io(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}
