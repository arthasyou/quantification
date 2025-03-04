use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{PartialSchema, ToSchema};

use crate::static_items::{position::Direction, strategy::StrategyConfig};

#[derive(Serialize, ToSchema, Debug)]
pub struct RiskData {
    pub symbol: String,
    pub direction: Direction,
    pub margin: String,
    pub entry_price: String,
    pub stop_price: String,
    pub quantity: String,
    #[schema(schema_with = String::schema)]
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetRiskResponse {
    pub code: u16,
    pub data: Vec<RiskData>,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetStategyResponse {
    pub code: u16,
    pub data: StrategyConfig,
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct UpdateStrategy {
    pub cfg: StrategyConfig,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct CreatePositionRequest {
    pub symbol: String,
    pub direction: Direction,
    pub leverage: u8,
    pub margin: f64,
    pub stop_loss_percent: f64,
    pub strategy_id: u8,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct ClosePositionRequest {
    pub symbol: String,
    pub direction: Direction,
}
