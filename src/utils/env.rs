use crate::utils::error_ext::MapS3Error;
use worker::{Result as CfResult, Var};

pub fn get_env_value<F>(getter: F, key: &str) -> Result<String, s3s::S3Error>
where
    F: Fn(&str) -> CfResult<Var>,
{
    getter(key).map(|v| v.to_string()).map_s3_error(
        s3s::S3ErrorCode::ServiceUnavailable,
        "Failed to retrieve env value",
    )
}
