// use s3s::auth::S3AuthContext;

use super::User;

#[derive(Clone)]
pub struct Context {
    pub user: Option<User>,
    // pub s3_context: &S3AuthContext<'a>,
}
