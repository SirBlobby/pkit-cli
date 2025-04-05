use crate::{
    filesystem,
    get_root_folder
};

use crate::filesystem::config::Config;
use crate::parser::ClICommand;
use crate::commands;

use crate::formatter::colorize;

use crate::api::{self, request};


pub async fn install_software(language: &str, version: &str) {

    let software: api::Version = api::get_language_version(language, version).await;

    println!("Downloading {} version {}...", software.language, software.version);
    
    let file_name = software.url.split("/").last().unwrap();

    let root_folder = get_root_folder();
    let archive_path = format!("{}/bin/{}/{}/{}", root_folder, software.language, software.version, file_name);
    
    request::download(&software.url, &archive_path)
    .await
    .unwrap();

    filesystem::extract(&archive_path);
    filesystem::delete(&archive_path);

    println!("Installed {} version {}.", software.language, software.version);

    println!("Do you want to set this version as the default? (y/n): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut config = Config::new();

    if input.trim() == "y" {
        config.add(&software.language, &software.version, &format!("/bin/{}/{}/", software.language, software.version), true);
        commands::default::add_pkit_path(&format!("{}/bin/{}/{}/", config.path, software.language, software.version), &software.language);
    } else {
        config.add(&software.language, &software.version, &format!("./bin/{}/{}/", software.language, software.version), false);
        std::process::exit(0);
    }
}

// pkit install <language> <version>

pub async fn main(cli: &ClICommand) {

    match cli.command.len() {
        2 => {
            install_software(&cli.command[0], &cli.command[1]).await;
        },
        _ => {
            println!("{}", colorize(&format!("Invalid number of arguments. Expected 2, got {}", cli.command.len())));
            std::process::exit(1);
        }
    }
    
}