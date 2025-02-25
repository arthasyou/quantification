use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::static_items::{position::Direction, strategy::StrategyConfig};

use super::biance_model::Risk;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetRiskResponse {
    pub code: u16,
    pub data: Vec<Risk>,
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
