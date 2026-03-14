use sqlx::PgPool;

use crate::error::AppError;

pub async fn count_today(db: &PgPool) -> Result<i64, AppError> {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM win_logs WHERE created_at >= CURRENT_DATE")
        .fetch_one(db)
        .await
        .map_err(|_| AppError::Internal)
}

pub async fn log_win(db: &PgPool, email: &str) -> Result<(), AppError> {
    sqlx::query("INSERT INTO win_logs (user_email) VALUES ($1)")
        .bind(email)
        .execute(db)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(())
}
