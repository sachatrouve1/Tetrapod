use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

fn get_current_date() -> String {
    let current_date = Local::now().format("%d-%m-%Y").to_string();
    return current_date;
}

pub fn get_cache_file_path(filename: &str) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".cache")
        .join("tetrapod")
        .join(filename)
}

pub fn is_up_to_date(filename: &str) -> bool {
    let date_file_path = get_cache_file_path(filename);

    match fs::read_to_string(&date_file_path) {
        Ok(stored_date) if stored_date.trim() == get_current_date() => true,

        _ => {
            if let Some(parent) = date_file_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Cannot create cache directory: {e}");
                    return false;
                }
            }
            false
        }
    }
}

pub fn record_date(filename: &str) -> Result<()> {
    let _ = fs::write(&get_cache_file_path(filename), get_current_date());
    Ok(())
}
