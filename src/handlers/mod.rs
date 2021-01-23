pub mod note;
use tide::Request;

pub async fn custom_404(_req: Request<()>) -> tide::Result {
    Ok("404 NOT FOUND".to_string().into())
}
