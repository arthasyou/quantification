use service_utils_rs::services::db::get_db;

use crate::error::Result;
use crate::models::auth_model::{Auth, AuthInput};

pub async fn create_auth_table() -> Result<()> {
    let query = "
        DEFINE TABLE IF NOT EXISTS auth SCHEMALESS PERMISSIONS FULL;
    
        DEFINE FIELD IF NOT EXISTS username ON TABLE auth TYPE string READONLY;
        DEFINE FIELD IF NOT EXISTS password ON TABLE auth TYPE string;
        DEFINE FIELD IF NOT EXISTS uuid ON TABLE auth TYPE string READONLY;
  
        DEFINE INDEX IF NOT EXISTS unique_uuid ON TABLE auth FIELDS uuid UNIQUE;
        DEFINE INDEX IF NOT EXISTS unique_username ON TABLE auth FIELDS username UNIQUE;
       ";

    let db = get_db();
    db.query(query).await?;
    Ok(())
}

pub async fn db_singup(input: AuthInput) -> Result<()> {
    let uuid: String = uuid::Uuid::new_v4().to_string();
    let user = Auth {
        uuid,
        username: input.username,
        password: input.password,
    };
    let db = get_db();
    let _r: Option<Auth> = db.create(("auth", &user.username)).content(user).await?;
    Ok(())
}

pub async fn get_auth(username: &str) -> Result<Option<Auth>> {
    let db = get_db();
    let r: Option<Auth> = db.select(("auth", username)).await?;
    println!("get user: {:?}", r);
    Ok(r)
}
