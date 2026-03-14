use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};

use crate::{error::AppError, state::AppState};

/// A validated bearer token extracted from the `Authorization` header.
/// Using this as an extractor in a handler guarantees the request is authenticated.
pub struct AuthenticatedToken(pub String);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedToken {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer ").map(str::to_string))
            .ok_or(AppError::NotAuthenticated)?;

        if !state.tokens.lock().unwrap().contains(&token) {
            return Err(AppError::AccessDenied);
        }

        Ok(AuthenticatedToken(token))
    }
}
