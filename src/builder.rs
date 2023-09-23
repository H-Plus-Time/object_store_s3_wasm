use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use aws_sdk_s3::Client;
use aws_types::sdk_config::Builder;

use crate::{error::Error, S3};

pub struct S3Builder {
    pub(crate) config: Builder,
    pub(crate) bucket: Option<String>,
}

impl S3Builder {
    pub fn build(self) -> Result<S3, Error> {
        let sdk_config = self.config.build();
        Ok(S3 {
            client: Arc::new(Client::new(&sdk_config)),
            bucket: self.bucket.ok_or(Error::Unknown)?,
        })
    }
}

impl Deref for S3Builder {
    type Target = Builder;
    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for S3Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}
