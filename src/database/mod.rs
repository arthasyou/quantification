pub mod auth_db;
pub mod fee_db;

use crate::error::Result;
use auth_db::create_users_table;
use fee_db::create_test_table;

pub async fn create_tables() -> Result<()> {
    create_users_table().await?;
    create_test_table().await?;
    Ok(())
}
