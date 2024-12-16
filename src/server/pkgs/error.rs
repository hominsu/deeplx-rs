use super::Json;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub enum Error {
    JsonRejection(JsonRejection),
    InvalidTagHandling,
    InternalServerError,
    DeepLSessionMissing,
    DeepLUnauthorized,
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
            Error::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            Error::InvalidTagHandling => (
                StatusCode::BAD_REQUEST,
                "Invalid tag_handling value. Allowed values are 'html' and 'xml'.".to_string(),
            ),
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Error::DeepLSessionMissing => {
                (StatusCode::UNAUTHORIZED, "No dl_session Found".to_string())
            }
            Error::DeepLUnauthorized => (
                StatusCode::UNAUTHORIZED,
                "Your account is not a Pro account. Please upgrade your account or switch to a different account.".to_string()
            ),
            Error::InvalidAccessToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid access token".to_string(),
            ),
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
