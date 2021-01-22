use sqlx::mysql::MySqlPool;
use tide::prelude::*;
use tide::Request;

pub async fn post_note(req: Request<MySqlPool>) -> tide::Result {
    Ok("".to_string().into())
}

pub async fn get_note(req: Request<MySqlPool>) -> tide::Result {
    let code = req.param("code")?.to_string();
    Ok("".to_string().into())
}

pub async fn custom_404(_req: Request<()>) -> tide::Result {
    Ok("".to_string().into())
}
