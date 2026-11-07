use anyhow::Result;
use std::process::Command;

mod apod_post;
mod downloader;
mod record;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint_url = "https://science.nasa.gov/wp-json/wp/v2/apod-basic";
    let picture_filename = "wallpaper.png";
    let date_record_filename = "last_date.txt";
    let current_date = record::get_current_date();
    let cached_date = record::get_cached_date(date_record_filename);

    if cached_date.as_deref() == Some(&current_date) {
        println!("Up to date with today's APOD");
        return Ok(());
    }

    println!("Asking Apod...");
    let response = reqwest::get(endpoint_url).await?;
    let posts: Vec<apod_post::ApodPost> = response.json().await?;

    if let Some(latest_post) = posts.first() {
        if cached_date.as_deref() == Some(&latest_post.date) {
            println!(
                "Today's picture not published yet by NASA (latest is {}). Will retry later.",
                latest_post.date
            );
            return Ok(());
        }

        println!(
            "New astronomical picture found: {} ({})",
            latest_post.title, latest_post.date
        );

        if let Some(image_url) = &latest_post.hdurl {
            let image_path = downloader::download_image(image_url, picture_filename).await?;
            let bash_cmd = if std::path::Path::new("/run/current-system/sw/bin/bash").exists() {
                "/run/current-system/sw/bin/bash"
            } else {
                "bash"
            };

            match Command::new(bash_cmd)
                .arg("./set_wallpaper.sh")
                .arg(image_path)
                .status()
            {
                Ok(status) if status.success() => {
                    println!("Wallpaper script finished successfully");
                    let _ = record::record_date(date_record_filename, current_date);
                }
                Ok(status) => {
                    eprintln!("set_wallpaper.sh failed with exit status: {status}");
                }
                Err(e) => {
                    eprintln!("Failed to run set_wallpaper.sh: {e}");
                }
            }
        }
    }
    Ok(())
}
