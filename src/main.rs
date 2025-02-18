mod database;
mod error;
mod handlers;
mod models;
mod routes;

use std::sync::Arc;

use database::{create_tables, fee_db::get_sum_fee};
use service_utils_rs::{
    services::{db::init_db, http::http_server, jwt::Jwt},
    settings::Settings,
};

#[tokio::main]
async fn main() {
    let settings = Settings::new("config/services.toml").unwrap();
    init_db(settings.surrealdb).await.unwrap();
    create_tables().await.unwrap();
    get_sum_fee("test").await.unwrap();

    let jwt = Arc::new(Jwt::new(settings.jwt));
    let router = routes::create_routes(jwt);
    http_server::start(settings.http.port, router)
        .await
        .unwrap();

    // println!("Hello, world!");
}
