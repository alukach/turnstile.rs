/// A backend written for Postgrest. Expects a very specific schema to be loaded onto
/// the database.
use postgrest::Postgrest as PostgrestClient;
use s3s::{
    auth::{S3Auth, S3AuthContext, SecretKey},
    dto::{self, ParseTimestampError},
    S3ErrorCode, S3Result,
};
use serde::Deserialize;

use crate::{db::Backend, utils::error_ext::MapS3Error};

#[derive(Clone)]
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
            .map_s3_error(
                S3ErrorCode::InternalError,
                "Failed to execute PostgREST query",
            )?;

        let buckets = res
            .json::<Vec<DbBucketRecord>>()
            .await
            .map_s3_error(
                S3ErrorCode::InternalError,
                "Failed to parse PostgREST response",
            )?
            .into_iter()
            .map(dto::Bucket::try_from)
            .collect::<Result<Vec<dto::Bucket>, dto::ParseTimestampError>>()
            .map_s3_error(S3ErrorCode::InternalError, "Failed to parse timestamp")?;

        Ok(dto::ListBucketsOutput {
            buckets: Some(buckets),
            owner: Some(dto::Owner {
                // TODO: Populate with information from authenticated user
                display_name: Some("dummy".to_string()),
                id: Some("dummy".to_string()),
            }),
        })
    }
}

#[async_trait::async_trait]
impl Backend for Postgrest {
    // NOTE: Must separate the controller from the Backend methods to avoid the
    // `Rc<RefCell<wasm_bindgen_futures::Inner>> cannot be sent between threads safely` error (https://github.com/cloudflare/workers-rs/issues/485)
    async fn list(&self) -> S3Result<dto::ListBucketsOutput> {
        self.list().await
    }
}

#[async_trait::async_trait]
impl S3Auth for Postgrest {
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey> {
        // TODO: Fetch secret from DB
        // Right now, the secret key is the reverse of the access key
        Ok(SecretKey::from(
            access_key.chars().rev().collect::<String>(),
        ))
    }

    async fn check_access(&self, _cx: &mut S3AuthContext<'_>) -> S3Result<()> {
        // TODO: Implement access control
        // Right now, we allow all requests
        S3Result::Ok(())
    }
}

#[derive(Deserialize)]
struct DbBucketRecord {
    slug: String,
    created_at: String,
}

impl TryFrom<DbBucketRecord> for dto::Bucket {
    type Error = ParseTimestampError;

    fn try_from(record: DbBucketRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            name: Some(record.slug),
            creation_date: Some(dto::Timestamp::parse(
                dto::TimestampFormat::DateTime,
                record.created_at.as_str(),
            )?),
        })
    }
}
