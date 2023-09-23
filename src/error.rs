use std::convert::Infallible;

use aws_sdk_s3::{error::SdkError, operation::head_object::HeadObjectError, primitives::SdkBody};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("S3 infallble")]
    S3Infallible(#[from] SdkError<Infallible, http::response::Response<SdkBody>>),
    #[error("S3 head object error")]
    S3Head(#[from] SdkError<HeadObjectError, http::response::Response<SdkBody>>),
    #[error("S3 conversion error")]
    S3Conversion(#[from] aws_smithy_types::date_time::ConversionError),
    #[error("unknown object store error")]
    Unknown,
}

impl From<Error> for object_store::Error {
    fn from(value: Error) -> Self {
        object_store::Error::Generic {
            store: "S3",
            source: Box::new(value),
        }
    }
}
