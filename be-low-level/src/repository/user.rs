use sqlx::PgPool;

use crate::error::AppError;

pub async fn get_password(db: &PgPool, email: &str) -> Result<Option<String>, AppError> {
    sqlx::query_scalar::<_, String>("SELECT password FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(db)
        .await
        .map_err(|_| AppError::Internal)
}
