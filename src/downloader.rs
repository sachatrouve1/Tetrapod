use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use anyhow::Result;

pub async fn download_image(url: &str, file_name: &str) -> Result<()> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let path = Path::new(file_name);
    let mut file = File::create(path).await?;

    file.write_all(&bytes).await?;

    Ok(())
}
