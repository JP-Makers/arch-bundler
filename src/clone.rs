use futures_util::StreamExt;
use reqwest::Client;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

pub async fn fetch_source(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    if source.starts_with("http://") || source.starts_with("https://") {
        let client = Client::new();
        let response = client.get(source).send().await?;

        // Try to get filename from URL
        let filename = source
            .split('/')
            .last()
            .unwrap_or("downloaded_file")
            .split('?')
            .next()
            .unwrap_or("downloaded_file")
            .to_string();

        let mut file = File::create(&filename).await?;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        println!("Download of {} complete!", filename);
        Ok(filename)
    } else {
        // Assume local path
        let path = Path::new(source);
        if !path.exists() {
            return Err(format!("Source path does not exist: {}", source).into());
        }

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid source path")?
            .to_string();

        // If it's already in the current directory, no need to copy
        if path.parent().map_or(true, |p| p.as_os_str().is_empty()) {
            return Ok(filename);
        }

        // Copy local file to current directory
        fs::copy(source, &filename).await?;
        println!("Copied {} to current directory", source);

        Ok(filename)
    }
}
