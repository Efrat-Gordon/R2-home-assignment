use axum::{extract::State, Json};
use serde_json::json;

use crate::{auth::AuthenticatedToken, error::AppError, state::AppState};

pub async fn logout(
    State(state): State<AppState>,
    AuthenticatedToken { token, .. }: AuthenticatedToken,
) -> Result<Json<serde_json::Value>, AppError> {
    state.tokens.lock().unwrap().remove(&token);
    Ok(Json(json!("OK")))
}
