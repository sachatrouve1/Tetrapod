use std::fs;
use std::path::PathBuf;
use chrono::Utc;

pub fn is_up_to_date(filename: &str) -> bool {
    let date_file_path = get_cache_file_path(filename);
    let current_date = Utc::now().format("%d-%m-%Y").to_string();

    match fs::read_to_string(&date_file_path) {
        Ok(stored_date) if stored_date.trim() == current_date => true,

        _ => {
            if let Some(parent) = date_file_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Cannot create cache directory: {e}");
                    return false;
                }
            }

            if let Err(e) = fs::write(&date_file_path, current_date) {
                eprintln!("Error while writing date file: {e}");
            }

            false
        }
    }
}

fn get_cache_file_path(filename: &str) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".cache")
        .join("tetrapod")
        .join(filename)
}
