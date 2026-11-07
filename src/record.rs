use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

pub fn get_cache_file_path(filename: &str) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".cache")
        .join("tetrapod")
        .join(filename)
}

pub fn get_current_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn get_cached_date(filename: &str) -> Option<String> {
    let date_file_path = get_cache_file_path(filename);
    fs::read_to_string(&date_file_path)
        .ok()
        .map(|s| s.trim().to_string())
}

pub fn record_date(filename: &str, date: String) -> Result<()> {
    let path = get_cache_file_path(filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, date)?;

    Ok(())
}
