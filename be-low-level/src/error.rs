use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

pub enum AppError {
    InvalidCredentials,
    NotAuthenticated,
    AccessDenied,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Incorrect email or password. Please try again.",
            ),
            AppError::NotAuthenticated => (
                StatusCode::UNAUTHORIZED,
                "You must be logged in to perform this action.",
            ),
            AppError::AccessDenied => (
                StatusCode::UNAUTHORIZED,
                "Access denied due to invalid credentials.",
            ),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occurred.",
            ),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
