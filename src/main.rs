use anyhow::Result;

mod apod_post;
mod downloader;


#[tokio::main]
async fn main() -> Result<()> {
    let endpoint_url = "https://science.nasa.gov/wp-json/wp/v2/apod-basic";
    let file_name = "apod_picture.png";

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

            downloader::download_image(image_url, file_name).await?;
        }
    } else {
        println!("No post found");
    }

    Ok(())
}

