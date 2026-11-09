use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

pub async fn download_image(url: &str, filename: PathBuf) -> Result<PathBuf> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    if let Some(parent) = filename.parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::write(&filename, bytes).await?;

    Ok(filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_download_image_creates_parent_directories() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                use tokio::io::AsyncReadExt;
                let mut buf = [0u8; 1024];
                let _ = socket.read(&mut buf).await;
                let http_response = "HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\ntest";
                let _ = socket.write_all(http_response.as_bytes()).await;
                let _ = socket.shutdown().await;
            }
        });

        let temp_dir = std::env::temp_dir().join(format!("tetrapod_test_{}", std::process::id()));
        let dest_file = temp_dir.join("sub_dir").join("test_image.png");

        assert!(!temp_dir.exists());

        let result = download_image(&format!("http://127.0.0.1:{port}"), dest_file.clone()).await;
        assert!(result.is_ok());
        assert!(dest_file.exists());
        assert_eq!(std::fs::read_to_string(&dest_file).unwrap(), "test");

        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
