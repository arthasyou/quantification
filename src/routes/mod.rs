mod auth_route;
mod fee_route;

use auth_route::{routes_auth, AuthApi};
use axum::{Extension, Router};
use fee_route::routes_test;
use service_utils_rs::services::{http::middleware::cors::create_cors, jwt::Jwt};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = "/auth", api = AuthApi)
        )
    )]
struct ApiDoc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    // let mut doc = ApiDoc::openapi();
    let cors = create_cors();

    let doc = ApiDoc::openapi();
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
        .nest("/auth", routes_auth())
        .nest("/test", routes_test())
        .layer(Extension(jwt))
        .layer(cors)
}
