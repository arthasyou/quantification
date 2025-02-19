use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::static_items::strategy::StrategyConfig;

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
