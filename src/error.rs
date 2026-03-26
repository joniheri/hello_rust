use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Internal(&'static str),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            AppError::Internal(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: "INTERNAL_ERROR",
                    message,
                },
            ),
        };

        (status, Json(body)).into_response()
    }
}
