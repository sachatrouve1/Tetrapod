use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApodPost {
    pub title: String,
    pub hdurl: Option<String>,
}
