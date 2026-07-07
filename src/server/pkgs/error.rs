use super::Json;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub enum Error {
    Deeplx(deeplx::Error),
    JsonRejection(JsonRejection),
    InternalServer,
    DeepLSessionMissing,
    InvalidAccessToken,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            code: u16,
            message: String,
        }

        let (code, message) = match self {
            Error::Deeplx(err) => (err.status_code(), err.to_string()),
            Error::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            Error::InternalServer => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Error::DeepLSessionMissing => {
                (StatusCode::UNAUTHORIZED, "No dl_session Found".to_string())
            }
            Error::InvalidAccessToken => {
                (StatusCode::UNAUTHORIZED, "Invalid access token".to_string())
            }
        };

        Json(ErrorResponse {
            code: code.as_u16(),
            message,
        })
        .with_status_code(code)
        .into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(value: JsonRejection) -> Self {
        Self::JsonRejection(value)
    }
}

impl From<deeplx::Error> for Error {
    fn from(value: deeplx::Error) -> Self {
        Self::Deeplx(value)
    }
}
