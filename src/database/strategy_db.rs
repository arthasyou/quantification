use service_utils_rs::services::db::get_db;

use crate::{
    error::{Error, Result},
    models::trade_model::UpdateStrategy,
    static_items::strategy::{StrategyConfig, UserStrategy},
};

pub async fn create_strategy_table() -> Result<()> {
    let query = "
        DEFINE TABLE IF NOT EXISTS strategy SCHEMALESS PERMISSIONS FULL;
    
        DEFINE FIELD IF NOT EXISTS user_id ON TABLE strategy TYPE string READONLY;
        DEFINE FIELD IF NOT EXISTS cfg ON TABLE strategy TYPE object;
  
        DEFINE INDEX IF NOT EXISTS unique_user_id ON TABLE strategy FIELDS user_id UNIQUE;
       ";

    let db = get_db();
    db.query(query).await?;
    Ok(())
}

pub async fn db_create_strategy(user_id: &str) -> Result<UserStrategy> {
    let input = UserStrategy {
        user_id: user_id.to_string(),
        cfg: StrategyConfig::default(),
    };
    let db = get_db();
    let r: Option<UserStrategy> = db.create(("strategy", user_id)).content(input).await?;
    match r {
        Some(r) => Ok(r),
        None => Err(Error::ErrorMessage("Create strategy failed".to_owned())),
    }
}

pub async fn db_get_strategy(user_id: &str) -> Result<UserStrategy> {
    let db = get_db();
    let r: Option<UserStrategy> = db.select(("strategy", user_id)).await?;
    match r {
        Some(r) => Ok(r),
        None => Err(Error::ErrorMessage("Get strategy failed".to_owned())),
    }
}

pub async fn db_update_strategy(user_id: &str, input: UpdateStrategy) -> Result<UserStrategy> {
    let db = get_db();
    let r: Option<UserStrategy> = db.update(("strategy", user_id)).content(input).await?;
    match r {
        Some(r) => Ok(r),
        None => Err(Error::ErrorMessage("Update strategy failed".to_owned())),
    }
}
