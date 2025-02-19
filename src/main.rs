mod biance;
mod database;
mod error;
mod handlers;
mod models;
mod routes;
mod static_items;

use std::sync::Arc;

use database::{
    create_tables, fee_db::get_sum_fee, strategy_db::db_create_strategy, user_db::db_update_player,
};
use service_utils_rs::{
    services::{db::init_db, http::http_server, jwt::Jwt},
    settings::Settings,
};
use static_items::price::get_price;

#[tokio::main]
async fn main() {
    let settings = Settings::new("config/services.toml").unwrap();
    init_db(settings.surrealdb).await.unwrap();
    create_tables().await.unwrap();
    db_update_player().await.unwrap();

    // let p = get_price();
    // println!("price: {:?}", p);

    let jwt = Arc::new(Jwt::new(settings.jwt));
    let router = routes::create_routes(jwt);
    http_server::start(settings.http.port, router)
        .await
        .unwrap();
}
