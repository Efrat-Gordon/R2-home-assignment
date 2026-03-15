use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, Json};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{LoginRequest, TokenResponse},
    repository::user,
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 401, description = "Invalid credentials"),
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let hash = user::get_password(&state.db, &body.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let password = body.password;
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let parsed = PasswordHash::new(&hash).map_err(|_| AppError::Internal)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AppError::InvalidCredentials)
    })
    .await
    .map_err(|_| AppError::Internal)??;

    let token = Uuid::new_v4().to_string();
    state
        .tokens
        .lock()
        .map_err(|_| AppError::Internal)?
        .insert(token.clone(), body.email);

    Ok(Json(TokenResponse { token }))
}
