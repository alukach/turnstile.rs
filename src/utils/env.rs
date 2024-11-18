use crate::utils::error_ext::MapS3Error;
use worker::{Env, Result as CfResult};

pub trait EnvExt {
    fn get_val(&self, key: &str) -> Result<String, s3s::S3Error>;
    fn get_secret(&self, key: &str) -> Result<String, s3s::S3Error>;
}

impl EnvExt for Env {
    fn get_val(&self, key: &str) -> Result<String, s3s::S3Error> {
        process_val(self.var(key))
    }
    fn get_secret(&self, key: &str) -> Result<String, s3s::S3Error> {
        process_val(self.secret(key))
    }
}

fn process_val<T>(val: CfResult<T>) -> Result<String, s3s::S3Error>
where
    T: ToString,
{
    val.map(|v| v.to_string()).map_s3_error(
        s3s::S3ErrorCode::ServiceUnavailable,
        "Failed to retrieve env value",
    )
}
