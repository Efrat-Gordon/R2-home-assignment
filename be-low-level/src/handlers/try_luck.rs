use axum::{extract::State, Json};
use rand::Rng;

use crate::{
    auth::AuthenticatedToken, cache, config, error::AppError, models::LuckResponse,
    repository::wins, state::AppState,
};

#[utoipa::path(
    post,
    path = "/api/try_luck",
    responses(
        (status = 200, description = "Luck result", body = LuckResponse),
        (status = 401, description = "Not authenticated or invalid token"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn try_luck(
    State(state): State<AppState>,
    AuthenticatedToken { email, .. }: AuthenticatedToken,
) -> Result<Json<LuckResponse>, AppError> {
    let mut redis = state.redis.clone();

    let daily_wins = match cache::get_wins(&mut redis).await {
        Ok(n) => n,
        Err(_) => wins::count_today(&state.db).await.unwrap_or(0),
    };

    let probability = if daily_wins >= config::DAILY_WIN_THRESHOLD as i64 {
        config::WIN_PROBABILITY_REDUCED
    } else {
        config::WIN_PROBABILITY_NORMAL
    };

    let win = rand::thread_rng().gen::<f64>() < probability;

    if win {
        wins::log_win(&state.db, &email).await?;
        let _ = cache::increment_wins(&mut redis).await;
    }

    Ok(Json(LuckResponse { win }))
}
