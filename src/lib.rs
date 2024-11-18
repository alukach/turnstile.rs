pub mod controller;
pub mod db;
mod utils;
// mod object_store;

use crate::controller::Controller;
use crate::utils::env::EnvExt;
use s3s::service::S3ServiceBuilder;
use tracing::{debug, error};
use worker::{event, Context, Env, HttpRequest};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> s3s::S3Result<http::Response<s3s::Body>> {
    console_error_panic_hook::set_once();

    // Configure Backends
    let api_url = env.get_val("API_URL")?;
    let api_key = env.get_secret("API_KEY")?;
    let db_backend = db::postgrest::Postgrest::new(api_url, api_key);
    let auth_backend = db_backend.clone();

    // Create S3S request handler
    let mut builder = S3ServiceBuilder::from(Controller::new(Box::new(db_backend)));
    builder.set_auth(auth_backend);

    // Handle request
    let res = builder
        .build()
        .call(req.map(|body| s3s::Body::http_body(body)))
        .await;
    match res {
        Ok(ref res) => debug!(?res),
        Err(ref err) => error!(?err),
    };
    res
}
