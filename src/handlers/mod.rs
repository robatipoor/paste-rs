pub mod note;
use tide::Request;

pub async fn custom_404(_req: Request<()>) -> tide::Result {
    Ok("".to_string().into())
}
