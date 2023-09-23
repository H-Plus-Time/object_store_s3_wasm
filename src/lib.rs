use std::fmt::Display;

use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_s3::Client;
use builder::S3Builder;
use bytes::Bytes;
use chrono::{DateTime, NaiveDateTime, Utc};
use error::Error;
use futures::stream::BoxStream;
use object_store::{ObjectMeta, ObjectStore};
use tokio::io::AsyncWrite;

pub mod builder;
mod error;

#[derive(Debug)]
pub struct S3 {
    client: Client,
    bucket: String,
}

impl S3 {
    pub fn builder() -> S3Builder {
        S3Builder {
            config: SdkConfig::builder(),
            bucket: None,
        }
    }
}

#[async_trait]
impl ObjectStore for S3 {
    async fn abort_multipart(
        &self,
        location: &object_store::path::Path,
        multipart_id: &object_store::MultipartId,
    ) -> object_store::Result<()> {
        unimplemented!()
    }
    async fn copy(
        &self,
        from: &object_store::path::Path,
        to: &object_store::path::Path,
    ) -> object_store::Result<()> {
        unimplemented!()
    }
    async fn copy_if_not_exists(
        &self,
        from: &object_store::path::Path,
        to: &object_store::path::Path,
    ) -> object_store::Result<()> {
        Err(object_store::Error::NotSupported {
            source: Box::new(Error::Unknown),
        })
    }
    async fn delete(&self, location: &object_store::path::Path) -> object_store::Result<()> {
        unimplemented!()
    }
    async fn get_opts(
        &self,
        location: &object_store::path::Path,
        options: object_store::GetOptions,
    ) -> object_store::Result<object_store::GetResult> {
        unimplemented!()
    }
    async fn head(
        &self,
        location: &object_store::path::Path,
    ) -> object_store::Result<object_store::ObjectMeta> {
        let output = self
            .client
            .head_object()
            .set_bucket(Some(self.bucket.clone()))
            .set_key(Some(location.to_string()))
            .send()
            .await
            .map_err(Error::from)?;
        let last_modified = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::from_timestamp_millis(
                output
                    .last_modified()
                    .ok_or(Error::Unknown)?
                    .to_millis()
                    .map_err(Error::from)?,
            )
            .ok_or(Error::Unknown)?,
            Utc,
        );
        let meta = ObjectMeta {
            location: location.clone(),
            last_modified,
            size: output.content_length() as usize,
            e_tag: output.e_tag().map(|x| x.to_string()),
        };
        Ok(meta)
    }
    async fn list(
        &self,
        prefix: Option<&object_store::path::Path>,
    ) -> object_store::Result<BoxStream<'_, object_store::Result<object_store::ObjectMeta>>> {
        unimplemented!()
    }
    async fn list_with_delimiter(
        &self,
        prefix: Option<&object_store::path::Path>,
    ) -> object_store::Result<object_store::ListResult> {
        unimplemented!()
    }
    async fn put(
        &self,
        location: &object_store::path::Path,
        bytes: Bytes,
    ) -> object_store::Result<()> {
        unimplemented!()
    }
    async fn put_multipart(
        &self,
        location: &object_store::path::Path,
    ) -> object_store::Result<(
        object_store::MultipartId,
        Box<dyn AsyncWrite + Unpin + Send>,
    )> {
        unimplemented!()
    }
}

impl Display for S3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.client.config())
    }
}
