use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;

use crate::handlers::user_handler::{create_user, get_user_info, logout};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::user_handler::create_user,
        crate::handlers::user_handler::get_user_info,
        crate::handlers::user_handler::logout,
    ),
    // components(schemas(ApiKeyAuth))
)]
pub struct UserApi;

pub fn routes_user() -> Router {
    Router::new()
        .route("/create", post(create_user))
        .route("/get_info", get(get_user_info))
        .route("/logout", post(logout))
}
