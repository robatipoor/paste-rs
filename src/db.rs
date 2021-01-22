use crate::config::CONFIG;
use sqlx::mysql::MySqlPool;
#[derive(Debug)]
pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new() -> crate::Result<Self> {
        let pool = MySqlPool::connect(&CONFIG.database_url).await?;
        Ok(Self { pool })
    }
}
