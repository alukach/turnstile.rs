use s3s::{dto, s3_error, S3Result};

use crate::models::Context;

pub mod postgrest;

#[async_trait::async_trait]
pub trait Backend: Send + Sync + 'static {
    async fn list(&self, _context: Context) -> S3Result<dto::ListBucketsOutput> {
        Err(s3_error!(NotImplemented, "list is not implemented yet"))
    }
}
