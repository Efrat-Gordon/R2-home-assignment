use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize, ToSchema)]
pub struct LuckResponse {
    pub win: bool,
}
