
use std::fs;

use reqwest;

pub async fn get(url: &str) {
    let client: reqwest::Client = reqwest::Client::new();

    let resp: reqwest::Response = client.get(url)
        .send()
        .await
        .expect("Failed to send request");

    println!("Status: {:?}", resp.status());
}

pub async fn post(url: &str) {  
    let client: reqwest::Client = reqwest::Client::new();

    let resp: reqwest::Response = client.post(url)
        .send()
        .await
        .expect("Failed to send request");

    println!("Status: {:?}", resp.status());
}

pub async fn download(url: &str) {
    let client: reqwest::Client = reqwest::Client::new();

    let resp: reqwest::Response = client.get(url)
        .send()
        .await
        .expect("Failed to send request");

    println!("Status: {:?}", resp.status());


    let bytes = resp.bytes().await.expect("Failed to read response");
    let filename: &str = url.split("/").last().unwrap().split("?").next().unwrap();
    fs::write(filename, bytes).expect("Failed to write file");
}