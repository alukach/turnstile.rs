// The Controller marshals S3 requests to the appropriate backends. It is where business
// logic will be applied.
use crate::db::Backend;
use s3s::{dto, service::S3ServiceBuilder, S3Request, S3Response, S3Result, S3};

pub struct Controller {
    db: Box<dyn Backend>,
}

impl Controller {
    pub fn from(db: Box<dyn Backend>) -> Self {
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

impl Into<S3ServiceBuilder> for Controller {
    fn into(self) -> S3ServiceBuilder {
        S3ServiceBuilder::new(self)
    }
}
