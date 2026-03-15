use axum::{extract::State, Json};
use serde_json::json;

use crate::{auth::AuthenticatedToken, error::AppError, state::AppState};

#[utoipa::path(
    post,
    path = "/api/logout",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Not authenticated or invalid token"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn logout(
    State(state): State<AppState>,
    AuthenticatedToken { token, .. }: AuthenticatedToken,
) -> Result<Json<serde_json::Value>, AppError> {
    state.tokens.lock().map_err(|_| AppError::Internal)?.remove(&token);
    Ok(Json(json!("OK")))
}
