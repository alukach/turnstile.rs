mod auth;
mod backend;
mod server;

use tracing::{debug, error};
use worker::{event, Context, Env, HttpRequest};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> s3s::S3Result<http::Response<s3s::Body>> {
    console_error_panic_hook::set_once();
    let service = server::setup();

    let res = service
        .call(req.map(|body| s3s::Body::http_body(body)))
        .await;

    match res {
        Ok(ref res) => debug!(?res),
        Err(ref err) => error!(?err),
    };

    res
}
