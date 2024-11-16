use s3s::{dto, s3_error, S3Result};

pub mod postgrest;

#[async_trait::async_trait]
pub trait Backend: Send + Sync + 'static {
    async fn list(&self) -> S3Result<dto::ListBucketsOutput> {
        Err(s3_error!(NotImplemented, "list is not implemented yet"))
    }
}
