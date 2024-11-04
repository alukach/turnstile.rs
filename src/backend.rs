use postgrest::Postgrest;
use s3s::{
    dto::{self, ParseTimestampError},
    S3Error, S3ErrorCode, S3Request, S3Response, S3Result, S3,
};
use serde::Deserialize;

pub struct DummyBackend {
    pub pg: Postgrest,
}

#[derive(Deserialize)]
struct BucketRecord {
    slug: String,
    created_at: String,
}

#[async_trait::async_trait]
impl S3 for DummyBackend {
    async fn list_buckets(
        &self,
        _req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let req = self
            .pg
            .from("bucket")
            .select("slug, created_at")
            .execute()
            .await
            .map_err(|_e| {
                S3Error::new(S3ErrorCode::InternalError)
                // .with_message(format!("Failed to execute PostgREST query: {}", e))
            })?;

        let db_buckets = req
            .json::<Vec<BucketRecord>>()
            .await
            .map_err(|_e| S3Error::new(S3ErrorCode::InternalError))?;

        let buckets = db_buckets
            .into_iter()
            .map(|record| {
                dto::Timestamp::parse(dto::TimestampFormat::DateTime, record.created_at.as_str())
                    .and_then(|ts| {
                        Ok(dto::Bucket {
                            name: Some(record.slug),
                            creation_date: Some(ts),
                        })
                    })
            })
            .collect::<Result<Vec<dto::Bucket>, ParseTimestampError>>()
            .map_err(|_e| S3Error::new(S3ErrorCode::InternalError))?;

        Ok(S3Response::new(dto::ListBucketsOutput {
            buckets: Some(buckets),
            owner: Some(dto::Owner {
                display_name: Some("dummy".to_string()),
                id: Some("dummy".to_string()),
            }),
        }))
        // Ok(S3Response::new(dto::ListBucketsOutput {
        //     buckets: None,
        //     owner: Some(dto::Owner {
        //         display_name: Some("dummy".to_string()),
        //         id: Some("dummy".to_string()),
        //     }),
        // }))
    }
}
