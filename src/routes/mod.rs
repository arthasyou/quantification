mod auth_route;
mod fee_route;
mod record_route;
mod trade_route;
mod user_route;

use auth_route::{routes_auth, AuthApi};
use axum::{middleware, Extension, Router};
use fee_route::routes_fee;
use record_route::{routes_record, RecordApi};
use service_utils_rs::services::{
    http::middleware::{auth_mw::auth, cors::create_cors},
    jwt::Jwt,
};
use std::sync::Arc;
use trade_route::{routes_trade, TradeApi};
use user_route::{routes_user, UserApi};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = "/auth", api = AuthApi),
            (path = "/user", api = UserApi),
            (path = "/trade", api = TradeApi),
            (path = "/record", api = RecordApi)
        ),
    )]
struct ApiDoc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    let cors = create_cors();
    let doc = ApiDoc::openapi();
    Router::new()
        .nest("/user", routes_user())
        .nest("/fee", routes_fee())
        .nest("/trade", routes_trade())
        .nest("/record", routes_record())
        .route_layer(middleware::from_fn(auth))
        .nest("/auth", routes_auth())
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
