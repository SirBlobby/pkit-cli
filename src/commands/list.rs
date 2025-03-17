use json;

use crate::{formatter::colorize, formatter::add_strings, formatter::capitalize_first};
use crate::request;

use crate::parser::ClICommand;

fn get_filters() -> String {
    let mut filters:String = String::new();

    filters.push_str("?platform=");
    if cfg!(target_os = "windows") { filters.push_str("win"); }
    if cfg!(target_os = "macos") { filters.push_str("darwin"); }
    if cfg!(target_os = "linux") { filters.push_str("linux"); }

    filters.push_str("&arch=");
    if cfg!(target_arch = "x86_64") && cfg!(target_pointer_width = "64") { 
        filters.push_str("x64"); 
    } else if cfg!(target_arch = "arm") && cfg!(target_pointer_width = "64") { 
        filters.push_str("arm64"); 
    } else {
        filters.push_str("none");
    }

    filters
}

async fn print_available_languages() {
    let res = request::get("https://pkit.sirblob.me/api/language").await;
    let json_data: json::JsonValue = json::parse(&res.text().await.unwrap()).unwrap();


    print!("{}", colorize(&add_strings(&["&a".to_string(), "Available Languages: ".to_string(), "&r".to_string() ])));

    for lang in json_data.members() {
        print!("{}", colorize(&add_strings(&[
            "&e".to_string(),
            capitalize_first(&lang.to_string()),
            "&r ".to_string(),
        ])));
    }
    println!();
    println!("{}", colorize(&add_strings(&["Use &3pkit list <languages>&r to see available versions for that language.".to_string()])));
}

async fn print_available_language_versions(language: &str) {
    let res: reqwest::Response = request::get(&format!("https://pkit.sirblob.me/api/language/{}{}", language.to_lowercase(), get_filters())).await;
    let json_data: json::JsonValue = json::parse(&res.text().await.unwrap()).unwrap();

    println!("{}", colorize(&add_strings(&["&a".to_string(), "Available Versions for ".to_string(), "&e".to_string(), capitalize_first(language), "&r".to_string() ])));

    for data in json_data.members() {
        println!("{}", colorize(&add_strings(&[
            "&a".to_string(),
            capitalize_first(language).to_string(),
            "&r - &e".to_string(),
            data["version"].to_string(),
            "&r - (&3".to_string(),
            data["platform"].to_string(),
            "&r, &5".to_string(),
            data["arch"].to_string(),
            "&r)".to_string(),
        ])));
    }
    println!();
    // println!("{}", colorize(&add_strings(&["Use &3pkit install <language>&r to install a specific version.".to_string()])));
}


pub async fn main(cli: &ClICommand) {

    let _ = cli;

    if cli.command.len() > 0 {
        print_available_language_versions(&cli.get_first()).await;
    } else {
        print_available_languages().await;
    }

}