use crate::{
    filesystem,
    get_root_folder
};

use crate::filesystem::config::Config;
use crate::commands;

use crate::formatter::{print_colored, print_success, print_info};

use crate::api::{self, request};

pub async fn install_software(language: &str, version: &str) {

    let software: api::Version = api::get_language_version(language, version).await;

    print_info(&format!("Downloading {} version {}...", software.language, software.version));
    
    let file_name = software.url.split("/").last().unwrap();

    let root_folder = get_root_folder();
    let archive_path = format!("{}/bin/{}/{}/{}", root_folder, software.language, software.version, file_name);
    
    request::download(&software.url, &archive_path)
    .await
    .unwrap();

    filesystem::extract(&archive_path);
    filesystem::delete(&archive_path);

    print_success(&format!("Installed {} version {}.", software.language, software.version));

    print_colored("&eDo you want to set this version as the default? (y/n): &r");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut config = Config::new();

    let default_path = format!("{}/bin/{}/{}/", get_root_folder(), software.language, software.version);
    if input.trim() == "y" {
        config.add(&software.language, &software.version, &default_path, true);
        commands::default::add_pkit_path(&default_path, &software.language);
    } else {
        config.add(&software.language, &software.version, &default_path, false);
        std::process::exit(0);
    }
}

pub async fn run_install(language: &str, version: &str) {
    install_software(language, version).await;
}

pub async fn handle_install_command(language: &str, version: &str) {
    run_install(language, version).await;
}