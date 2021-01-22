use crate::config::CONFIG;
use crate::handler::*;
use sqlx::mysql::MySqlPool;

pub async fn run() -> crate::Result {
    let pool = MySqlPool::connect(&CONFIG.database_url).await?;
    let mut app = tide::with_state(pool);
    app.at("/").post(post_note);
    app.at("/:code").post(get_note);
    app.at("/assets").serve_dir("assets/")?;
    app.listen(CONFIG.base_url.as_str()).await?;
    Ok(())
}
