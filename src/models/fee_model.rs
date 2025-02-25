use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
// use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug)]
pub struct Fee {
    pub user_id: String,
    pub agent_id: String,
    pub amount: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateFeeRequest {
    pub user_id: String,
    pub agent_id: String,
    pub amount: Decimal,
}
