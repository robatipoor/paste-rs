use crate::services::note;
use log::error;
use sqlx::mysql::MySqlPool;
use tide::StatusCode;
use tide::{Request, Response};

pub async fn post(mut req: Request<MySqlPool>) -> tide::Result {
    let data = req.body_string().await?;
    let pool = req.state();
    match note::save(pool, data).await {
        Ok(code) => Ok(code.into()),
        Err(e) => {
            error!("post note error {}", e);
            let mut resp = Response::new(StatusCode::InternalServerError);
            resp.set_body("save note failed!");
            Ok(resp)
        }
    }
}

pub async fn get(req: Request<MySqlPool>) -> tide::Result {
    let pool = req.state();
    let code = req.param("code")?;
    match note::find_one(pool, code).await {
        Ok(n) => Ok(n.data.into()),
        Err(e) => {
            error!("get note error {}", e);
            let mut resp = Response::new(StatusCode::InternalServerError);
            resp.set_body("note not found !");
            Ok(resp)
        }
    }
}

pub async fn update(mut req: Request<MySqlPool>) -> tide::Result {
    let data = req.body_string().await?;
    let pool = req.state();
    let code = req.param("code")?;
    match note::update(pool, code, data).await {
        Ok(_) => Ok("update success !".into()),
        Err(e) => {
            error!("update note error {}", e);
            let mut resp = Response::new(StatusCode::InternalServerError);
            resp.set_body("update note failed!");
            Ok(resp)
        }
    }
}

pub async fn delete(req: Request<MySqlPool>) -> tide::Result {
    let pool = req.state();
    let code = req.param("code")?;
    match note::delete(pool, code).await {
        Ok(_) => Ok("delete note success".into()),
        Err(e) => {
            error!("delete note error {}", e);
            let mut resp = Response::new(StatusCode::InternalServerError);
            resp.set_body("save note failed!");
            Ok(resp)
        }
    }
}
