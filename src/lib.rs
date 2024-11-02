mod backend;
use crate::backend::DummyBackend;

use s3s::service::S3ServiceBuilder;
use tracing::{debug, error};
use worker::{event, Context, Env, HttpRequest};

async fn run(req: http::Request<s3s::Body>) -> s3s::S3Result<http::Response<s3s::Body>> {
    let service = {
        let backend = DummyBackend {};
        let builder = S3ServiceBuilder::new(backend);
        builder.build()
    };

    let result = service.call(req).await;

    match result {
        Ok(ref res) => debug!(?res),
        Err(ref err) => error!(?err),
    };

    result
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> s3s::S3Result<http::Response<s3s::Body>> {
    console_error_panic_hook::set_once();
    let s3_req = req.map(|body| s3s::Body::http_body(body));
    run(s3_req).await
}
