use axum::{extract::State, Json};
use chrono::Local;
use rand::Rng;

use crate::{
    auth::AuthenticatedToken,
    config,
    error::AppError,
    models::LuckResponse,
    state::AppState,
};

pub async fn try_luck(
    State(state): State<AppState>,
    AuthenticatedToken(_): AuthenticatedToken,
) -> Result<Json<LuckResponse>, AppError> {
    let mut daily = state.daily_wins.lock().unwrap();

    // Reset the counter when the day rolls over
    let today = Local::now().date_naive();
    if daily.date != today {
        daily.date = today;
        daily.count = 0;
    }

    let probability = if daily.count >= config::DAILY_WIN_THRESHOLD {
        config::WIN_PROBABILITY_REDUCED
    } else {
        config::WIN_PROBABILITY_NORMAL
    };

    let win = rand::thread_rng().gen::<f64>() < probability;

    if win {
        daily.count += 1;
    }

    Ok(Json(LuckResponse { win }))
}
