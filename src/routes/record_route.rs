use crate::handlers::record_handler::get_positions;
use axum::{routing::get, Router};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(crate::handlers::record_handler::get_positions))]
pub struct RecordApi;

pub fn routes_record() -> Router {
    Router::new().route("/get_positions", get(get_positions))
}
