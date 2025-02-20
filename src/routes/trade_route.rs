use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;

use crate::handlers::trade_handler::{create_position, get_risk, get_strategy, update_strategy};

#[derive(OpenApi)]
#[openapi(paths(
    crate::handlers::trade_handler::get_risk,
    crate::handlers::trade_handler::get_strategy,
    crate::handlers::trade_handler::update_strategy,
    crate::handlers::trade_handler::create_position
))]
pub struct TradeApi;

pub fn routes_trade() -> Router {
    Router::new()
        .route("/get_risk", get(get_risk))
        .route("/get_strategy", get(get_strategy))
        .route("/update_strategy", post(update_strategy))
        .route("/create_position", post(create_position))
}
