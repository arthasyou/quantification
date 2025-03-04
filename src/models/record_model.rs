use crate::static_items::position::Direction;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetPositionsRequest {
    pub symbol: String,
    pub dirction: Direction,
}
