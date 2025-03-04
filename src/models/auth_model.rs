use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug)]
pub struct Auth {
    pub username: String,
    pub password: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Login {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub refresh: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub code: u16,
    pub data: Login,
    pub message: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
}
