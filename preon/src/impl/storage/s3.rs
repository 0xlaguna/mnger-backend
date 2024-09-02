use std::{path::PathBuf, time::Duration};

use aws_config::Region;
use aws_sdk_s3::{
    primitives::ByteStream,
    Client,
    presigning::PresigningConfig
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

    pub async fn delete_object(key: String) -> Result<()> {
        let config = aws_config::from_env().region(Region::new("us-east-1")).load().await;
        let client = Client::new(&config);
        let bucket = std::env::var("AWS_BUCKET")
            .map_err(|e| Error::InternalError { info: e.to_string()})?;

        let _request = client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e|Error::InternalError {
                info: e.to_string()
            })?;

        Ok(())
    }

    pub async fn generate_presigned_url(key: String) -> Result<String> {
        let config = aws_config::from_env().region(Region::new("us-east-1")).load().await;
        let client = Client::new(&config);
        let bucket = std::env::var("AWS_BUCKET")
            .map_err(|e| Error::InternalError { info: e.to_string()})?;

        let presign_config = PresigningConfig::expires_in(Duration::from_secs(3600))
            .map_err(|e| Error::InternalError { info: e.to_string()})?;

        let request = client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(presign_config)
            .await
            .map_err(|e|Error::InternalError {
                info: e.to_string()
            })?;

        let signed_url = request.uri().to_string();

        Ok(signed_url)
    }

    pub fn extract_filename(file_path: Option<String>) -> Option<String> {
        if let Some(avatar) = file_path {
            let parts: Vec<&str> = avatar.split('/').collect();

            return parts.last().cloned().map(str::to_string);
        }
        None
    }
}
