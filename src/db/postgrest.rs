/// A backend written for Postgrest. Expects a very specific schema to be loaded onto
/// the database.
use crate::models::User;
use postgrest::Postgrest as PostgrestClient;
use s3s::{
    auth::{Credentials, S3Auth, S3AuthContext, SecretKey},
    dto::{self, ParseTimestampError},
    S3ErrorCode, S3Result,
};
use serde::Deserialize;

use crate::{db::Backend, models::Context, utils::error_ext::MapS3Error};

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
    async fn list(&self, ctx: Context) -> S3Result<dto::ListBucketsOutput> {
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
            owner: match ctx.user {
                Some(user) => Some(dto::Owner {
                    display_name: Some(user.username),
                    id: Some(user.id),
                }),
                None => None,
            },
        })
    }
}

#[async_trait::async_trait]
impl Backend for Postgrest {
    // NOTE: Must separate the controller from the Backend methods to avoid the
    // `Rc<RefCell<wasm_bindgen_futures::Inner>> cannot be sent between threads safely` error (https://github.com/cloudflare/workers-rs/issues/485)
    async fn list(&self, ctx: Context) -> S3Result<dto::ListBucketsOutput> {
        self.list(ctx).await
    }
}

#[async_trait::async_trait]
impl S3Auth for Postgrest {
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey> {
        // NOTE: Fetch with service-level auth token
        // let Ok(secret) = self
        //     .db
        //     .from("key")
        //     .select("secret")
        //     .eq("id", access_key)
        //     .execute()
        //     .await
        // else {
        //     return Err(S3ErrorCode::InternalError.into());
        // };
        Ok(SecretKey::from(
            access_key.chars().rev().collect::<String>(),
        ))
    }

    async fn check_access(&self, _cx: &mut S3AuthContext<'_>) -> S3Result<()> {
        // TODO: Implement access control
        // - 1. Check DB for relationship between key & target
        // - 2. Check CEL logic (?)

        // TODO: If access is granted, the user will be stored in context
        // https://blog.adamchalmers.com/what-are-extensions/
        let ctx = Context {
            user: Some(User {
                id: "fake-id".to_string(),
                username: "fake-username".to_string(),
            }),
            // s3_context: *_cx,
        };
        _cx.extensions_mut().insert(ctx);

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
