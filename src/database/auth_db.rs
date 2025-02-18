use service_utils_rs::services::db::get_db;

use crate::error::Result;
use crate::models::auth_model::{User, UserInput};

pub async fn create_users_table() -> Result<()> {
    let query = "
        DEFINE TABLE IF NOT EXISTS user SCHEMALESS PERMISSIONS FULL;
    
        DEFINE FIELD IF NOT EXISTS username ON TABLE user TYPE string;
        DEFINE FIELD IF NOT EXISTS password ON TABLE user TYPE string;
        DEFINE FIELD IF NOT EXISTS uuid ON TABLE user TYPE string;
  
        DEFINE INDEX IF NOT EXISTS unique_uuid ON TABLE user FIELDS uuid UNIQUE;
        DEFINE INDEX IF NOT EXISTS unique_username ON TABLE user FIELDS username UNIQUE;
       ";

    let db = get_db();
    db.query(query).await?;
    Ok(())
}

pub async fn create_user(input: UserInput) -> Result<()> {
    let uuid: String = uuid::Uuid::new_v4().to_string();
    let user = User {
        uuid,
        username: input.username,
        password: input.password,
    };
    let db = get_db();
    let _r: Option<User> = db.create(("user", &user.username)).content(user).await?;
    Ok(())
}

pub async fn get_user(username: &str) -> Result<Option<User>> {
    let db = get_db();
    let r: Option<User> = db.select(("user", username)).await?;
    println!("get user: {:?}", r);
    Ok(r)
}
