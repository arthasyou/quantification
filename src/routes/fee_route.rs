use axum::{routing::post, Router};

use crate::handlers::fee_handler::create_fee;

pub fn routes_test() -> Router {
    Router::new().route("/create_fee", post(create_fee))
}
