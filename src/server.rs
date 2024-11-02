use crate::auth::Auth;
use crate::backend::DummyBackend;

use s3s::service::{S3Service, S3ServiceBuilder};

pub fn setup() -> S3Service {
    let backend = DummyBackend {};
    let auth = Auth {};

    let mut builder = S3ServiceBuilder::new(backend);
    builder.set_auth(auth);
    builder.build()
}
