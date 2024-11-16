pub mod auth;
pub mod controller;
pub mod db;
// mod object_store;

use auth::AuthBackend;
use controller::Controller;
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

    // Build backend
    let db_backend = db::postgrest::Postgrest::new(
        env.var("API_URL")
            .map_err(|e| {
                s3s::S3Error::with_message(
                    s3s::S3ErrorCode::ServiceUnavailable,
                    format!("Failed to generate service config: {}", e),
                )
            })?
            .to_string(),
        env.secret("API_KEY")
            .map_err(|e| {
                s3s::S3Error::with_message(
                    s3s::S3ErrorCode::ServiceUnavailable,
                    format!("Failed to generate service config: {}", e),
                )
            })?
            .to_string(),
    );
    let backend = Controller::new(Box::new(db_backend));

    // Assemble S3S request handler
    let mut builder = S3ServiceBuilder::new(backend);
    builder.set_auth(AuthBackend {});
    let s3_service = builder.build();

    // Handle request
    let res = s3_service
        .call(req.map(|body| s3s::Body::http_body(body)))
        .await;
    match res {
        Ok(ref res) => debug!(?res),
        Err(ref err) => error!(?err),
    };
    res
}
