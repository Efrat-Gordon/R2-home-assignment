use axum::{extract::State, Json};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{LoginRequest, TokenResponse},
    repository::user,
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let password = user::get_password(&state.db, &body.email).await?;

    let valid = password.map(|pwd| pwd == body.password).unwrap_or(false);

    if !valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = Uuid::new_v4().to_string();
    state.tokens.lock().unwrap().insert(token.clone(), body.email);

    Ok(Json(TokenResponse { token }))
}
