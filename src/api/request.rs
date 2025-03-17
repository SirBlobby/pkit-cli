
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::stream::StreamExt;

use reqwest;

pub async fn get(url: &str) -> reqwest::Response {
    let client: reqwest::Client = reqwest::Client::new();

    let resp: reqwest::Response = client.get(url)
        .send()
        .await
        .expect("Failed to send request");
    resp
}

pub async fn post(url: &str) {  
    let client: reqwest::Client = reqwest::Client::new();

    let resp: reqwest::Response = client.post(url)
        .send()
        .await
        .expect("Failed to send request");

    println!("Status: {:?}", resp.status());
}


pub async fn download(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {

    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        return Err(Box::from(format!("Failed to download: {}", response.status())));
    }
    
    let total_size = response
        .content_length()
        .unwrap_or(0);
    
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{wide_bar} {bytes}/{total_bytes} ({eta})")? 
            .progress_chars("##-")
    );
    
    let mut file = File::create(path)?;
    let mut downloaded = 0;
    
    let mut stream = response.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }
    
    pb.finish_with_message("Download complete!");
    Ok(())
}
