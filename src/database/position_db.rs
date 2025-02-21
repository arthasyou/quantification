use service_utils_rs::services::db::get_db;

use crate::error::Result;

pub async fn create_position_table() -> Result<()> {
    let query = "
    DEFINE TABLE IF NOT EXISTS positon SCHEMALESS PERMISSIONS FULL;

    DEFINE FIELD IF NOT EXISTS order_id ON TABLE positon TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS user_id ON TABLE positon TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS symbol ON TABLE positon TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS key ON TABLE user TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS secret ON TABLE user TYPE string READONLY;
    DEFINE FIELD IF NOT EXISTS created_at ON TABLE positon VALUE time::now() READONLY;

    DEFINE INDEX IF NOT EXISTS order_id_index ON TABLE positon FIELDS order_id;
    DEFINE INDEX IF NOT EXISTS user_id_index ON TABLE positon FIELDS user_id;
    DEFINE INDEX IF NOT EXISTS created_at_index ON TABLE positon FIELDS created_at;
   ";

    let db = get_db();
    db.query(query).await?;
    Ok(())
}
