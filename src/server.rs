use crate::auth::Auth;
use crate::backend::DummyBackend;

use postgrest::Postgrest;
use s3s::service::{S3Service, S3ServiceBuilder};
use worker::{Env, Result};

pub struct Config {
    postgrest_api: String,
    postgrest_key: String,
}

impl Config {
    /// Create a new Config from the Cloudflare environment.
    pub fn from_cf_env(env: Env) -> Result<Self> {
        let postgrest_api = env.var("SUPABASE_API_URL")?;
        let postgrest_key = env.secret("SUPABASE_API_KEY")?;
        Ok(Self {
            postgrest_api: postgrest_api.to_string(),
            postgrest_key: postgrest_key.to_string(),
        })
    }

    pub fn build_service(self) -> S3Service {
        let pg = Postgrest::new(self.postgrest_api).insert_header("apikey", self.postgrest_key);
        let backend = DummyBackend { pg };
        let auth = Auth {};

        let mut builder = S3ServiceBuilder::new(backend);
        builder.set_auth(auth);
        builder.build()
    }
}
