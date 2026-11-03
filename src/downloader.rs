use tokio::fs;
use anyhow::Result;

pub async fn download_image<'a>(url: &'a str, filename: &'a str) -> Result<&'a str> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    fs::write(&filename, bytes).await?;

    println!("Image saved at: {}", filename);

    Ok(filename)
}
