use axum::{routing::post, Router};
use axum_test::TestServer;
use be_low_level::{handlers, state::AppState};
use serde_json::json;

fn test_server() -> TestServer {
    let state = AppState::new();
    let app = Router::new()
        .route("/api/login", post(handlers::login::login))
        .route("/api/logout", post(handlers::logout::logout))
        .route("/api/try_luck", post(handlers::try_luck::try_luck))
        .with_state(state);
    TestServer::new(app).unwrap()
}

// ── login ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn login_valid_credentials_returns_token() {
    let server = test_server();
    let res = server
        .post("/api/login")
        .json(&json!({ "email": "a@gmail.com", "password": "1234" }))
        .await;

    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert!(body["token"].as_str().is_some());
}

#[tokio::test]
async fn login_wrong_password_returns_401() {
    let server = test_server();
    let res = server
        .post("/api/login")
        .json(&json!({ "email": "a@gmail.com", "password": "wrong" }))
        .await;

    res.assert_status_unauthorized();
}

#[tokio::test]
async fn login_unknown_email_returns_401() {
    let server = test_server();
    let res = server
        .post("/api/login")
        .json(&json!({ "email": "nobody@example.com", "password": "1234" }))
        .await;

    res.assert_status_unauthorized();
}

// ── logout ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn logout_valid_token_returns_ok() {
    let server = test_server();

    let login_res = server
        .post("/api/login")
        .json(&json!({ "email": "a@gmail.com", "password": "1234" }))
        .await;
    let token = login_res.json::<serde_json::Value>()["token"]
        .as_str()
        .unwrap()
        .to_string();

    let res = server
        .post("/api/logout")
        .add_header("Authorization", format!("Bearer {token}"))
        .await;

    res.assert_status_ok();
}

#[tokio::test]
async fn logout_without_token_returns_401() {
    let server = test_server();
    let res = server.post("/api/logout").await;
    res.assert_status_unauthorized();
}

#[tokio::test]
async fn logout_invalid_token_returns_401() {
    let server = test_server();
    let res = server
        .post("/api/logout")
        .add_header("Authorization", "Bearer not-a-real-token")
        .await;
    res.assert_status_unauthorized();
}

// ── try_luck ───────────────────────────────────────────────────────────

#[tokio::test]
async fn try_luck_returns_win_field() {
    let server = test_server();

    let login_res = server
        .post("/api/login")
        .json(&json!({ "email": "a@gmail.com", "password": "1234" }))
        .await;
    let token = login_res.json::<serde_json::Value>()["token"]
        .as_str()
        .unwrap()
        .to_string();

    let res = server
        .post("/api/try_luck")
        .add_header("Authorization", format!("Bearer {token}"))
        .await;

    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert!(body["win"].as_bool().is_some());
}

#[tokio::test]
async fn try_luck_without_token_returns_401() {
    let server = test_server();
    let res = server.post("/api/try_luck").await;
    res.assert_status_unauthorized();
}

#[tokio::test]
async fn try_luck_invalid_token_returns_401() {
    let server = test_server();
    let res = server
        .post("/api/try_luck")
        .add_header("Authorization", "Bearer not-a-real-token")
        .await;
    res.assert_status_unauthorized();
}
