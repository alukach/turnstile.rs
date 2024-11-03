mod auth;
mod backend;
// mod object_store;
mod server;

use server::Config;
use tracing::{debug, error};
use worker::{event, Context, Env, HttpRequest};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> s3s::S3Result<http::Response<s3s::Body>> {
    console_error_panic_hook::set_once();
    let Ok(service_config) = Config::from_cf_env(env) else {
        return Err(s3s::S3Error::new(s3s::S3ErrorCode::InternalError));
    };
    let s3_service = service_config.build_service();

    let res = s3_service
        .call(req.map(|body| s3s::Body::http_body(body)))
        .await;

    match res {
        Ok(ref res) => debug!(?res),
        Err(ref err) => error!(?err),
    };

    res
}
