use object_store::{path::Path, ObjectStore};
use s3s::{dto, s3_error, stream, S3Request, S3Response, S3Result, S3};
use std::sync::Arc;

/// A struct that implements the S3 trait using object_store
pub struct ObjectStoreS3 {
    object_store: Arc<dyn ObjectStore>,
}

impl ObjectStoreS3 {
    /// Creates a new instance of ObjectStoreS3
    pub fn new(object_store: Arc<dyn ObjectStore>) -> Self {
        Self { object_store }
    }
}

#[async_trait::async_trait]
impl S3 for ObjectStoreS3 {
    async fn get_object(
        &self,
        req: S3Request<dto::GetObjectInput>,
    ) -> S3Result<S3Response<dto::GetObjectOutput>> {
        let input = req.input;
        let bucket = input.bucket;
        let key = input.key;

        // Construct the path
        let path = Path::from(format!("{}/{}", bucket, key));

        // Get the object from the object store
        match self.object_store.get(&path).await {
            Ok(mut res) => {
                // let mut data: Vec<_> = Vec::new();
                // // let stream = get_result.into_stream().await?;
                // while let Ok(chunk) = res.bytes().await {
                //     data.extend_from_slice(&chunk);
                // }

                let body = res.into_stream() else { todo!() };

                let output = dto::GetObjectOutput {
                    body: Some(body),
                    // Populate other fields as necessary
                    ..Default::default()
                };
                Ok(S3Response::new(output))
            }
            Err(e) => Err(s3_error!(NoSuchKey, "Object not found: {}", e)),
        }
    }

    async fn put_object(
        &self,
        req: S3Request<dto::PutObjectInput>,
    ) -> S3Result<S3Response<dto::PutObjectOutput>> {
        let input = req.input;
        let bucket = input.bucket;
        let key = input.key;
        let body = input.body;

        let path = Path::from(format!("{}/{}", bucket, key));

        // Put the object into the object store
        match self.object_store.put(&path, body.into()).await {
            Ok(_) => {
                let output = dto::PutObjectOutput {
                    // Populate fields as necessary
                    ..Default::default()
                };
                Ok(S3Response::new(output))
            }
            Err(e) => Err(s3_error!(InternalError, "Failed to put object: {}", e)),
        }
    }

    async fn list_objects(
        &self,
        req: S3Request<dto::ListObjectsInput>,
    ) -> S3Result<S3Response<dto::ListObjectsOutput>> {
        let input = req.input;
        let bucket = input.bucket;
        let prefix = input.prefix.unwrap_or_default();

        let path_prefix = format!("{}/{}", bucket, prefix);
        let path = Path::from(path_prefix);

        let mut stream = self
            .object_store
            .list(Some(&path))
            .await
            .map_err(|e| s3_error!(InternalError, "Failed to list objects: {}", e))?;

        let mut contents = Vec::new();

        while let Some(result) = stream.next().await {
            match result {
                Ok(object_meta) => {
                    let key = object_meta.location.to_string();
                    // Remove the bucket prefix from the key
                    let key = key
                        .strip_prefix(&format!("{}/", bucket))
                        .unwrap_or(&key)
                        .to_string();

                    let s3_object = dto::Object {
                        key,
                        size: object_meta.size as i64,
                        last_modified: Some(object_meta.last_modified),
                        // Populate other fields as necessary
                        ..Default::default()
                    };
                    contents.push(s3_object);
                }
                Err(e) => return Err(s3_error!(InternalError, "Failed to list objects: {}", e)),
            }
        }

        let output = dto::ListObjectsOutput {
            contents: Some(contents),
            // Populate other fields as necessary
            ..Default::default()
        };

        Ok(S3Response::new(output))
    }

    // For methods not directly supported by object_store, retain the default NotImplemented response
    // You can implement additional methods as needed using the object_store API
}
