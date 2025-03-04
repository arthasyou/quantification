use axum::{routing::post, Router};
use utoipa::OpenApi;

use crate::handlers::auth_handler::{login, refresh_access_token, signup};

#[derive(OpenApi)]
#[openapi(paths(
    crate::handlers::auth_handler::signup,
    crate::handlers::auth_handler::login,
    crate::handlers::auth_handler::refresh_access_token
))]
pub struct AuthApi;

pub fn routes_auth() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/refresh", post(refresh_access_token))
}
