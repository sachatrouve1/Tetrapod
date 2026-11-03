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

    if !record::is_up_to_date(date_record_filename) {
        println!("Asking Apod...");

        let response = reqwest::get(endpoint_url).await?;
        let posts: Vec<apod_post::ApodPost> = response.json().await?;

        if posts.is_empty() {
            println!("No data received");
            return Ok(());
        }

        if let Some(latest_post) = posts.first() {
            println!("Today's astronomical picture: {}", latest_post.title);

            if let Some(image_url) = &latest_post.hdurl {
                println!("Image url: {}", image_url);

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
                        let _ = record::record_date(date_record_filename);
                    }
                    Ok(status) => {
                        eprintln!("set_wallpaper.sh failed with exit status: {status}");
                    }
                    Err(e) => {
                        eprintln!("Failed to run set_wallpaper.sh: {e}");
                    }
                }
            }
        } else {
            println!("No post found");
        }
    }
    println!("Up to date");

    Ok(())
}

