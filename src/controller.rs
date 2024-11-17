/// The Controller delivers S3 requests to the appropriate backends, applying appropriate
/// business logic along the way.
use crate::db::Backend;
use s3s::{dto, service::S3ServiceBuilder, S3Request, S3Response, S3Result, S3};

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
        _req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let buckets = self.db.list().await?;
        Ok(S3Response::new(buckets))
    }
}

impl From<Controller> for S3ServiceBuilder {
    fn from(controller: Controller) -> Self {
        Self::new(controller)
    }
}
