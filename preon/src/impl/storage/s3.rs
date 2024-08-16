use std::path::PathBuf;

use aws_config::Region;
use aws_sdk_s3::{
    primitives::ByteStream,
    Client,
};
use clap::Parser;

use crate::{Result, Error};

pub struct S3;

#[derive(Debug, Parser)]
pub struct Opt {
    #[structopt(long)]
    pub key: String,
    #[structopt(long)]
    pub source: PathBuf,
}

impl S3 {
    pub async fn put_object(opts: &Opt) -> Result<()> {
        let config = aws_config::from_env().region(Region::new("us-east-1")).load().await;
        let client = Client::new(&config);
        let bucket = std::env::var("AWS_BUCKET")
            .map_err(|e| Error::InternalError { info: e.to_string()})?;

        let body = ByteStream::read_from()
            .path(opts.source.clone())
            .buffer_size(2048)
            .build()
            .await
            .map_err(|e|Error::InternalError {
                info: e.to_string()
            })?;

        let _request = client
            .put_object()
            .bucket(bucket)
            .key(opts.key.clone())
            .body(body)
            .send()
            .await
            .map_err(|e|Error::InternalError {
                info: e.to_string()
            })?;

        Ok(())
    }
}
