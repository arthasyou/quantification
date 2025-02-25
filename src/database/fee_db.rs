use rust_decimal::Decimal;
use service_utils_rs::services::db::get_db;

use crate::{
    error::Result,
    models::fee_model::{CreateFeeRequest, Fee},
};

pub async fn create_fee_table() -> Result<()> {
    let query = "
    DEFINE TABLE IF NOT EXISTS fee SCHEMALESS PERMISSIONS FULL;

    DEFINE FIELD IF NOT EXISTS user_id ON TABLE fee TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS agent_id ON TABLE fee TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS amount ON TABLE fee TYPE decimal READONLY;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE fee VALUE time::now() READONLY;

    DEFINE INDEX IF NOT EXISTS user_id_index ON TABLE fee FIELDS user_id;
    DEFINE INDEX IF NOT EXISTS agent_id_index ON TABLE fee FIELDS agent_id;
    DEFINE INDEX IF NOT EXISTS created_at_index ON TABLE fee FIELDS created_at;
   ";

    let db = get_db();
    db.query(query).await?;
    Ok(())
}

pub async fn db_create_fee(input: CreateFeeRequest) -> Result<()> {
    let db = get_db();
    let _r: Option<Fee> = db.create("fee").content(input).await?;
    Ok(())
}

pub async fn get_sum_fee(agnet_id: &str) -> Result<()> {
    // math::sum()
    let db = get_db();
    // let query = "SELECT amount FROM fee;";
    let query = "RETURN math::sum(SELECT Value amount FROM fee);";
    let mut r = db.query(query).await?;
    let a: Option<Decimal> = r.take(0)?;
    println!("get user: {:?}", a);
    Ok(())
}
