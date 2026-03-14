use axum::{extract::State, Json};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{LoginRequest, TokenResponse},
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let valid = state
        .users
        .get(&body.email)
        .map(|pwd| pwd == &body.password)
        .unwrap_or(false);

    if !valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = Uuid::new_v4().to_string();
    state.tokens.lock().unwrap().insert(token.clone());

    Ok(Json(TokenResponse { token }))
}
