mod biance;
mod database;
mod error;
mod handlers;
mod models;
mod routes;
mod static_items;
mod utils;
mod websocket;

use std::sync::Arc;

use database::create_tables;
use dotenvy::dotenv;
use service_utils_rs::{
    services::{db::init_db, http::http_server, jwt::Jwt},
    settings::Settings,
};
use static_items::percision::init_percisions;
use websocket::connection::start_websocket;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let settings = Settings::new("config/services.toml").unwrap();
    init_db(settings.surrealdb).await.unwrap();
    create_tables().await.unwrap();
    init_percisions().await;

    let jwt = Arc::new(Jwt::new(settings.jwt));
    let router = routes::create_routes(jwt);
    let http_task = http_server::start(settings.http.port, router);
    let ws_task = start_websocket();
    let _ = tokio::join!(ws_task, http_task);
}
