use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub tokens: Arc<Mutex<HashMap<String, String>>>, // token → email
    pub redis: MultiplexedConnection,
}
