use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInput {
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
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub refresh: String,
}
