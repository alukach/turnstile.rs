use s3s::{
    auth::{S3Auth, S3AuthContext, SecretKey},
    S3Result,
};

pub struct AuthBackend {}

#[async_trait::async_trait]
impl S3Auth for AuthBackend {
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey> {
        // TODO: Fetch secret from DB
        // Right now, the secret key is the reverse of the access key
        Ok(SecretKey::from(access_key.chars().rev().collect::<String>()))
    }

    async fn check_access(&self, _cx: &mut S3AuthContext<'_>) -> S3Result<()> {
        // TODO: Implement access control
        // Right now, we allow all requests
        S3Result::Ok(())
    }
}
