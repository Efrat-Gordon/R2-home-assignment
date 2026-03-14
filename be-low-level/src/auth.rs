use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{error::AppError, state::AppState};

/// A validated bearer token. Carries both the token and the owner's email.
/// Using this as an extractor in a handler guarantees the request is authenticated.
pub struct AuthenticatedToken {
    pub token: String,
    pub email: String,
}

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

        let email = state
            .tokens
            .lock()
            .unwrap()
            .get(&token)
            .cloned()
            .ok_or(AppError::AccessDenied)?;

        Ok(AuthenticatedToken { token, email })
    }
}
