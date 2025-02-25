use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{PartialSchema, ToSchema};
// use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct User {
    pub user_id: String,
    pub agent_id: String,
    pub balance: String,
    pub key: String,
    pub secret: String,
    #[schema(schema_with = String::schema)]
    pub created_at: DateTime<Utc>,
    #[schema(schema_with = String::schema)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub agent_id: String,
    pub key: String,
    pub secret: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateUserInput {
    pub user_id: String,
    pub agent_id: String,
    pub key: String,
    pub secret: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub code: u16,
    pub data: User,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserInfoRequest {
    pub balance: f64,
}
