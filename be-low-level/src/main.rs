mod auth;
mod config;
mod error;
mod handlers;
mod models;
mod state;

use axum::{routing::post, Router};
use state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new();

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
