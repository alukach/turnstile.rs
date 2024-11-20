use crate::models::User;
/// The Controller delivers S3 requests to the appropriate backends, applying appropriate
/// business logic along the way.
use crate::{db::Backend, models::Context};
use s3s::{dto, service::S3ServiceBuilder, S3Error, S3Request, S3Response, S3Result, S3};

pub struct Controller {
    db: Box<dyn Backend>,
}

impl Controller {
    pub fn new(db: Box<dyn Backend>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl S3 for Controller {
    /// List buckets that the keys are permitted to view
    async fn list_buckets(
        &self,
        req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let user = req.extensions.get::<User>();

        let Some(creds) = req.credentials else {
            return Err(S3Error::with_message(
                s3s::S3ErrorCode::CredentialsNotSupported,
                "Credentials must be provided",
            ));
        };
        let buckets = self
            .db
            .list(Context {
                user: user.cloned(),
                // s3_context: req.
            })
            .await?;
        Ok(S3Response::new(buckets))
    }
}

impl From<Controller> for S3ServiceBuilder {
    fn from(controller: Controller) -> Self {
        Self::new(controller)
    }
}
