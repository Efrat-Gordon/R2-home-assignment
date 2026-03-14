use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct LuckResponse {
    pub win: bool,
}
