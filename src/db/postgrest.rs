use postgrest::Postgrest as PostgrestClient;
use s3s::{dto, S3Error, S3ErrorCode, S3Result};
use serde::Deserialize;

use crate::db::Backend;

pub struct Postgrest {
    pub db: PostgrestClient,
}

impl Postgrest {
    pub fn new(api: String, secret: String) -> Self {
        Self {
            db: PostgrestClient::new(api).insert_header("apikey", secret),
        }
    }

    #[worker::send] // https://github.com/cloudflare/workers-rs/issues/485#issuecomment-2008599314
    async fn list(&self) -> S3Result<dto::ListBucketsOutput> {
        let res = self
            .db
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

        let db_buckets = res.json::<Vec<DbBucketRecord>>().await.map_err(|e| {
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
            .collect::<Result<Vec<dto::Bucket>, dto::ParseTimestampError>>()
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
}

#[async_trait::async_trait]
impl Backend for Postgrest {
    async fn list(&self) -> S3Result<dto::ListBucketsOutput> {
        // TODO: I have no idea why this doesn't cause infinite recursion
        self.list().await
    }
}

#[derive(Deserialize)]
struct DbBucketRecord {
    slug: String,
    created_at: String,
}
