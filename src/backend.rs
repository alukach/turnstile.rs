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

#[worker::send] // https://github.com/cloudflare/workers-rs/issues/485#issuecomment-2008599314
pub async fn list(postgrest: Postgrest) -> S3Result<dto::ListBucketsOutput> {
    let res = postgrest
        .from("bucket")
        .select("slug, created_at")
        .execute()
        .await
        .map_err(|e| {
            S3Error::with_message(
                S3ErrorCode::InternalError,
                format!("Failed to execute PostgREST query: {e:?}"),
            )
        })?;

    let db_buckets = res.json::<Vec<BucketRecord>>().await.map_err(|e| {
        S3Error::with_message(
            S3ErrorCode::InternalError,
            format!("Failed to parse PostgREST response: {e:?}"),
        )
    })?;

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
        .map_err(|e| {
            S3Error::with_message(
                S3ErrorCode::InternalError,
                format!("Failed to parse timestamp: {e:?}"),
            )
        })?;

    Ok(dto::ListBucketsOutput {
        buckets: Some(buckets),
        owner: Some(dto::Owner {
            display_name: Some("dummy".to_string()),
            id: Some("dummy".to_string()),
        }),
    })
}

#[async_trait::async_trait]
impl S3 for DummyBackend {
    async fn list_buckets(
        &self,
        _req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let buckets = list(self.pg.clone()).await?;
        Ok(S3Response::new(buckets))
    }
}
