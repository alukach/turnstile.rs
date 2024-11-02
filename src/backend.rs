use s3s::{dto, S3Error, S3ErrorCode, S3Request, S3Response, S3Result, S3};

pub struct DummyBackend {}

#[async_trait::async_trait]
impl S3 for DummyBackend {
    async fn list_buckets(
        &self,
        _req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        // Create a list of dummy buckets
        let mut buckets = vec![];
        let bucket_count = 10;
        for n in 1..bucket_count {
            let Ok(ts) =
                dto::Timestamp::parse(dto::TimestampFormat::DateTime, "2021-01-01T00:00:00Z")
            else {
                return Err(S3Error::new(S3ErrorCode::InternalError));
            };
            buckets.push(dto::Bucket {
                creation_date: Some(ts),
                name: Some(format!("dummy-bucket-{}", n)),
            });
        }

        Ok(S3Response::new(dto::ListBucketsOutput {
            buckets: Some(buckets),
            owner: Some(dto::Owner {
                display_name: Some("dummy".to_string()),
                id: Some("dummy".to_string()),
            }),
        }))
    }
}
