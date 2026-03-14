use axum::{extract::State, Json};
use rand::Rng;

use crate::{
    auth::AuthenticatedToken,
    config,
    error::AppError,
    models::LuckResponse,
    repository::wins,
    state::AppState,
};

pub async fn try_luck(
    State(state): State<AppState>,
    AuthenticatedToken { email, .. }: AuthenticatedToken,
) -> Result<Json<LuckResponse>, AppError> {
    let daily_wins = wins::count_today(&state.db).await?;

    let probability = if daily_wins >= config::DAILY_WIN_THRESHOLD as i64 {
        config::WIN_PROBABILITY_REDUCED
    } else {
        config::WIN_PROBABILITY_NORMAL
    };

    let win = rand::thread_rng().gen::<f64>() < probability;

    if win {
        wins::log_win(&state.db, &email).await?;
    }

    Ok(Json(LuckResponse { win }))
}
