use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

pub async fn download_image<'a>(url: &'a str, filename: PathBuf) -> Result<PathBuf> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    fs::write(&filename, bytes).await?;

    Ok(filename)
}
