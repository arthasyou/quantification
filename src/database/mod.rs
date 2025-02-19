pub mod auth_db;
pub mod fee_db;
pub mod flow_db;
pub mod strategy_db;
pub mod user_db;

use crate::error::Result;
use auth_db::create_auth_table;
use fee_db::create_fee_table;
use strategy_db::create_strategy_table;
use user_db::create_user_table;

pub async fn create_tables() -> Result<()> {
    create_auth_table().await?;
    create_fee_table().await?;
    create_user_table().await?;
    create_strategy_table().await?;
    Ok(())
}
