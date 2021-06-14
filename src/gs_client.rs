use std::{fs::File, path::PathBuf};
use cloud_storage::Client;
use std::io::Read;

const MIME_TYPE: &str = "application/json";


pub async fn upload_to_gs(gcloud_client: &Client, local_file: &PathBuf, bucket: &String, key: &String) -> Result<(), Box<dyn std::error::Error>> {
    let bytes =
        File::open(local_file)?
            .bytes()
            .map(|byte| byte.unwrap())
            .collect::<Vec<_>>();

    gcloud_client.object().create(bucket, bytes, key, MIME_TYPE).await?;

    Ok(())
}