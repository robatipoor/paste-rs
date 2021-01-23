use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{query, query_as};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Note {
    pub id: i32,
    pub code: String,
    pub time_created: DateTime<Utc>,
    pub time_updated: Option<DateTime<Utc>>,
    pub data: String,
}

impl Note {
    pub fn new(id: i32, code: String, data: String) -> Self {
        Note {
            id,
            code,
            time_created: Utc::now(),
            time_updated: None,
            data,
        }
    }

    pub async fn find_by_id(pool: &MySqlPool, id: &str) -> crate::Result<Self> {
        let sql = r#"SELECT * FROM paste WHERE id = ?"#;
        let rec = query_as::<_, Note>(sql).bind(id).fetch_one(pool).await?;
        Ok(rec)
    }

    pub async fn find_by_code(pool: &MySqlPool, code: &str) -> crate::Result<Self> {
        let sql = r#"SELECT * FROM paste WHERE code = ?"#;
        let rec = query_as::<_, Note>(sql).bind(code).fetch_one(pool).await?;
        Ok(rec)
    }

    pub async fn find_all(pool: &MySqlPool) -> crate::Result<Vec<Self>> {
        let sql = r#"SELECT * FROM paste"#;
        let recs = query_as::<_, Note>(sql).fetch_all(pool).await?;
        Ok(recs)
    }

    pub async fn insert(&self, pool: &MySqlPool) -> crate::Result<()> {
        let sql = r#"INSERT INTO paste (code,data,time_created,time_updated) VALUES(?,?,?,NULL)"#;
        query(sql)
            .bind(&self.code)
            .bind(&self.data)
            .bind(&self.time_created)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_by_code(pool: &MySqlPool, code: &str) -> crate::Result<()> {
        let sql = r#"DELETE FROM paste WHERE code = ?"#;
        query(sql).bind(code).execute(pool).await?;
        Ok(())
    }

    pub async fn update_by_code(
        pool: &MySqlPool,
        code: &str,
        data: String,
        time_update: DateTime<Utc>,
    ) -> crate::Result<()> {
        let sql = r#"UPDATE paste SET data = ? ,time_updated = ? WHERE code = ? "#;
        query(sql)
            .bind(data)
            .bind(time_update)
            .bind(code)
            .execute(pool)
            .await?;
        Ok(())
    }
}
