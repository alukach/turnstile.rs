//
use crate::db::Backend;
use s3s::{dto, S3Request, S3Response, S3Result, S3};

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
    async fn list_buckets(
        &self,
        _req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let buckets = self.db.list().await?;
        Ok(S3Response::new(buckets))
    }
}
