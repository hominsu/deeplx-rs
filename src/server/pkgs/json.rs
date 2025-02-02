use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(super::Error))]
pub struct Json<T>(pub T);

impl<T> Json<T> {
    pub fn with_status_code(self, status_code: StatusCode) -> WithStatusCode<T> {
        WithStatusCode {
            status_code,
            value: self,
        }
    }
}

impl<T> IntoResponse for Json<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

pub struct WithStatusCode<T> {
    pub status_code: StatusCode,
    pub value: Json<T>,
}

impl<T> IntoResponse for WithStatusCode<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        (self.status_code, self.value).into_response()
    }
}
