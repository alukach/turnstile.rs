use std::fmt::Debug;

use s3s::{S3Error, S3ErrorCode};

pub trait MapS3Error<T> {
    fn map_s3_error(self, code: S3ErrorCode, context: &str) -> Result<T, S3Error>;
}

impl<T, E: Debug> MapS3Error<T> for Result<T, E> {
    fn map_s3_error(self, code: S3ErrorCode, context: &str) -> Result<T, S3Error> {
        self.map_err(|e| S3Error::with_message(code.clone(), format!("{}: {:?}", context, e)))
    }
}
