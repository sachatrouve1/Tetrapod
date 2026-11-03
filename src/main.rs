use anyhow::Result;
use std::process::Command;

mod apod_post;
mod downloader;
mod date_checker;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint_url = "https://science.nasa.gov/wp-json/wp/v2/apod-basic";
    let picture_filename = "wallpaper.png";
    let date_record_filename = "last_date.txt";

    if !date_checker::is_up_to_date(date_record_filename) {
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

                let _status = Command::new("./set_wallpaper.sh")
                    .arg(image_path)
                    .status();
            }
        } else {
            println!("No post found");
        }
    }
    println!("Up to date");

    Ok(())
}

