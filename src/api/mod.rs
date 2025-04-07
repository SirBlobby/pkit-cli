
use std::env::consts::{OS, ARCH};

use json;

pub mod request;

const API_URL: &str = "https://pkit.sirblob.co/api";

pub struct Version {
    pub language: String,
    pub version: String,
    pub platform: String,
    pub arch: String,
    pub url: String
}

pub struct Language {
    pub name: String,
    pub versions: Vec<Version>
}

fn get_filters() -> String {
    let mut filters:String = String::new();

    filters.push_str("?platform=");
    if OS == "windows" { filters.push_str("win"); }
    if OS == "macos" { filters.push_str("darwin"); }
    if OS == "linux" { filters.push_str("linux"); }

    filters.push_str("&arch=");
    if ARCH == "x86_64" { filters.push_str("x64"); }
    else if (ARCH == "arm" && cfg!(target_pointer_width = "64")) || ARCH == "aarch64" { filters.push_str("arm64"); }
    else { filters.push_str("none"); }

    filters
}

pub async fn get_languages() -> Vec<String> {
    let res = request::get(&format!("{}/language", API_URL)).await;
    let json_data: json::JsonValue = json::parse(&res.text().await.unwrap()).unwrap();

    let mut languages: Vec<String> = Vec::new();
    for lang in json_data.members() {
        languages.push(lang.to_string());
    }

    languages
}

pub async fn get_language(language: &str) -> Language {
    let res = request::get(&format!("{}/language/{}{}", API_URL, language.to_lowercase(), get_filters())).await;
    let json_data: json::JsonValue = json::parse(&res.text().await.unwrap()).unwrap();

    let mut versions: Vec<Version> = Vec::new();
    for data in json_data.members() {
        versions.push(Version {
            language: data["name"].to_string(),
            version: data["version"].to_string(),
            platform: data["platform"].to_string(),
            arch: data["arch"].to_string(),
            url: data["url"].to_string()
        });
    }

    Language {
        name: language.to_string(),
        versions
    }
}

pub async fn get_language_version(language: &str, version: &str) -> Version {
    let res = request::get(&format!("{}/language/{}/{}{}", API_URL, language.to_lowercase(), version, get_filters())).await;
    let json_data: json::JsonValue = json::parse(&res.text().await.unwrap()).unwrap();

    Version {
        language: json_data["name"].to_string(),
        version: json_data["version"].to_string(),
        platform: json_data["platform"].to_string(),
        arch: json_data["arch"].to_string(),
        url: json_data["url"].to_string()
    }
}


// pub async fn download_language_version(language: &str, version: &str) {

// }