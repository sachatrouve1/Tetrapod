use serde::Deserialize;
// use std::fs::File;
// use std::io::Write;


#[derive(Deserialize, Debug)]
struct ApodPost {
    title: String,
    url: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://science.nasa.gov/wp-json/wp/v2/apod-basic";

    println!("Asking Apod...");

    let response = reqwest::get(url).await?;
    let posts: Vec<ApodPost> = response.json().await?;

    if posts.is_empty() {
        println!("No data received");
        return Ok(());
    }

    if let Some(latest_post) = posts.first() {
        println!("Today's astronomical picture: {}", latest_post.title);
        if let Some(image_url) = &latest_post.url {
            println!("Image url: {}", image_url);
        }
    } else {
        println!("No post found");
    }

    Ok(())
}

