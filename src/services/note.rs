use crate::config::CONFIG;
use crate::models::note::Note;
use crate::utils::random_string;
use chrono::Utc;
use sqlx::mysql::MySqlPool;

pub async fn save(pool: &MySqlPool, data: String) -> crate::Result<String> {
    let code = random_string(CONFIG.random_code_len);
    let note = Note::new(0, code.clone(), data);
    note.insert(pool).await?;
    Ok(code)
}

pub async fn find_one(pool: &MySqlPool, code: &str) -> crate::Result<Note> {
    let note = Note::find_by_code(pool, code).await?;
    Ok(note)
}

pub async fn update(pool: &MySqlPool, code: &str, data: String) -> crate::Result<()> {
    Note::update_by_code(pool, code, data, Utc::now()).await?;
    Ok(())
}

pub async fn delete(pool: &MySqlPool, code: &str) -> crate::Result<()> {
    Note::delete_by_code(pool, code).await?;
    Ok(())
}
