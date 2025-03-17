
use crate::filesystem;
use crate::parser::ClICommand;

use crate::api::{self, request};


pub async fn install_software(language: &str, version: &str) {

    let software: api::Version = api::get_language_version(language, version).await;

    println!("Downloading {} version {}...", software.language, software.version);
    
    let file_name = software.url.split("/").last().unwrap();
    
    request::download(&software.url, &format!("./bin/{}/{}/{}", software.language, software.version, file_name))
        .await
        .unwrap();

    let archive_path = format!("./bin/{}/{}/{}", software.language, software.version, file_name);
    filesystem::extract(&archive_path);
    filesystem::delete(&archive_path);

    println!("Installed {} version {}.", software.language, software.version);
}

// pkit install <language> <version>

pub async fn main(cli: &ClICommand) {

    match cli.command.len() {
        0 => {
            println!("No language specified.");
        },
        1 => {
            println!("No version specified.");
        },
        2 => {
            install_software(&cli.command[0], &cli.command[1]).await;
        },
        _ => {
            println!("Too many arguments.");
        }
    }
    

}