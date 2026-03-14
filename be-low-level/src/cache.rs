use chrono::Local;
use redis::{aio::MultiplexedConnection, AsyncCommands};

use crate::error::AppError;

fn wins_key() -> String {
    format!("daily:wins:{}", Local::now().date_naive())
}

pub async fn get_wins(conn: &mut MultiplexedConnection) -> Result<i64, AppError> {
    let val: Option<i64> = conn.get(wins_key()).await.map_err(|_| AppError::Internal)?;
    Ok(val.unwrap_or(0))
}

pub async fn increment_wins(conn: &mut MultiplexedConnection) -> Result<(), AppError> {
    let key = wins_key();
    let count: i64 = conn
        .incr(&key, 1i64)
        .await
        .map_err(|_| AppError::Internal)?;
    if count == 1 {
        // First win today — set a 48-hour TTL so the key auto-expires
        let _: i64 = conn
            .expire(&key, 48 * 3600)
            .await
            .map_err(|_| AppError::Internal)?;
    }
    Ok(())
}
