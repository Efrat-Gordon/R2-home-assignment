use axum::{routing::post, Router};
use be_low_level::{config, handlers, state::AppState};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let state = AppState {
        db,
        tokens: Arc::new(Mutex::new(HashSet::new())),
    };

    let app = Router::new()
        .route("/api/login", post(handlers::login::login))
        .route("/api/logout", post(handlers::logout::logout))
        .route("/api/try_luck", post(handlers::try_luck::try_luck))
        .with_state(state);

    let addr = format!("{}:{}", config::HOST, config::PORT);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
