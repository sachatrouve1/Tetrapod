use std::path::PathBuf;
use tokio::fs;
use anyhow::Result;

pub async fn download_image(url: &str, filename: &str) -> Result<()> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let dest_dir = PathBuf::from(home).join(".local");

    fs::create_dir_all(&dest_dir).await?;

    let file_path = dest_dir.join(filename);

    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    fs::write(&file_path, bytes).await?;

    println!("Image saved at: {}", file_path.display());

    Ok(())
}
