use axum::{routing::post, Router};
use utoipa::OpenApi;

use crate::handlers::auth_handler::{login, signup};

#[derive(OpenApi)]
#[openapi(paths(
    crate::handlers::auth_handler::signup,
    crate::handlers::auth_handler::login
))]
pub struct AuthApi;

pub fn routes_auth() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}
