use rust_decimal::Decimal;
use service_utils_rs::services::db::get_db;

use crate::{
    error::{Error, Result},
    models::user_model::{CreateUserInput, UpdateUserInfoRequest, UserInfo},
};

pub async fn create_user_table() -> Result<()> {
    let query = "
    DEFINE TABLE IF NOT EXISTS user SCHEMALESS PERMISSIONS FULL;

    DEFINE FIELD IF NOT EXISTS user_id ON TABLE user TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS agent_id ON TABLE user TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS balance ON TABLE user TYPE decimal DEFAULT 0;
    DEFINE FIELD IF NOT EXISTS key ON TABLE user TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS secret ON TABLE user TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE user VALUE time::now() READONLY;
    DEFINE FIELD IF NOT EXISTS updated_at ON TABLE user VALUE time::now();

    DEFINE INDEX IF NOT EXISTS user_id_index ON TABLE user FIELDS user_id;
    DEFINE INDEX IF NOT EXISTS agent_id_index ON TABLE user FIELDS agent_id;
   ";

    let db = get_db();
    db.query(query).await?;
    Ok(())
}

pub async fn db_create_user(input: CreateUserInput) -> Result<UserInfo> {
    let db = get_db();
    let r: Option<UserInfo> = db.create(("user", &input.user_id)).content(input).await?;
    match r {
        Some(user) => Ok(user),
        None => Err(Error::ErrorMessage("create user failed".to_owned())),
    }
}

pub async fn db_get_user_info(user_id: &str) -> Result<UserInfo> {
    let db = get_db();
    let r: Option<UserInfo> = db.select(("user", user_id)).await?;
    match r {
        Some(user) => Ok(user),
        None => Err(Error::ErrorMessage("get user failed".to_owned())),
    }
}

pub async fn db_update_player() -> Result<()> {
    let db = get_db();
    let input = UpdateUserInfoRequest {
        // user_id: "1".to_string(),
        // agent_id: "3".to_string(),
        balance: 11.3,
        // updated_at: chrono::Utc::now(),
    };
    let r: Option<UserInfo> = db.update(("user", "1")).content(input).await?;
    println!("create player: {:?}", r);
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
