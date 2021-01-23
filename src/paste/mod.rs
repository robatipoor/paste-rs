use crate::config::CONFIG;
use crate::handlers::note::*;
use sqlx::mysql::MySqlPool;
use std::io::ErrorKind;

use log::info;
use tide::utils::After;
use tide::{Response, StatusCode};

pub async fn run() -> crate::Result {
    env_logger::init();
    let pool = MySqlPool::connect(&CONFIG.database_url).await?;
    let mut app = tide::with_state(pool);
    app.with(After(|mut res: Response| async {
        if let Some(err) = res.downcast_error::<async_std::io::Error>() {
            if let ErrorKind::NotFound = err.kind() {
                let msg = format!("error: {:?}", err);
                res.set_status(StatusCode::NotFound);
                res.set_body(msg);
            }
        }
        Ok(res)
    }));
    app.at("/").post(post);
    app.at("/:code").get(get);
    app.at("/:code").put(update);
    app.at("/:code").delete(delete);
    app.at("/assets").serve_dir("assets/")?;
    info!("run app {}", CONFIG.base_url);
    app.listen(CONFIG.base_url.as_str()).await?;
    Ok(())
}
