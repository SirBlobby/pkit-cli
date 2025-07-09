use crate::{
    api::{self, request},
    filesystem::{self, config::Config, get_pkit_dir},
    formatter::{capitalize_first, colorize, print_box, BoxAlignment, BoxOptions},
};
use std::path::PathBuf;

async fn install_software_with_data(software: api::Version) {

    println!();
    let title = format!("&aDownloading &e{} {}&r", capitalize_first(&software.language), software.version);
    print_box(&[(title.as_str(), BoxAlignment::Center)], &BoxOptions::default());
    println!();

    let file_name = software.url.split('/').last().unwrap_or("download.tmp");

    let pkit_dir: PathBuf = get_pkit_dir().expect("Failed to get pkit directory");
    let archive_path = pkit_dir
        .join("bin")
        .join(&software.language)
        .join(&software.version)
        .join(file_name);

    if let Err(e) = request::download(&software.url, archive_path.to_str().unwrap()).await {
        println!();
        print_box(&[("&cDownload Failed&r", BoxAlignment::Center)], &BoxOptions::default());
        println!();
        println!("{}", colorize(&format!("  &cError: {}&r", e)));
        println!();
        return;
    }

    filesystem::extract(&archive_path).expect("Failed to extract archive");
    filesystem::delete(&archive_path).expect("Failed to delete archive");

    println!();
    print_box(&[("&aInstallation Complete&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();
    println!("{}", colorize(&format!("  &e{} {}&r has been successfully installed.", capitalize_first(&software.language), software.version)));
    println!();

    println!("{}", colorize("  &eDo you want to set this version as the default? (y/n): &r"));
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut config = Config::new();
    let default_path = pkit_dir
        .join("bin")
        .join(&software.language)
        .join(&software.version);

    if input.trim().eq_ignore_ascii_case("y") {
        config.add(&software.language, &software.version, default_path.to_str().unwrap(), true);
        config.write_env_script().expect("Failed to write environment script");
        
        println!();
        print_box(&[("&aSuccess&r", BoxAlignment::Center)], &BoxOptions::default());
        println!();
        println!("{}", colorize(&format!("  &e{} {}&r is now the default version.", capitalize_first(&software.language), software.version)));
        println!();
        
    } else {
        config.add(&software.language, &software.version, default_path.to_str().unwrap(), false);
        config.write_env_script().expect("Failed to write environment script");
        println!();
        let usage_line = format!(" &3pkit default {} {}&r  &8-&r  Set this version as default later", software.language, software.version);
        print_box(
            &[(usage_line.as_str(), BoxAlignment::Left)],
            &BoxOptions {
                title: Some("Next Steps"),
                ..Default::default()
            },
        );
        println!();
    }
}

pub async fn handle_install_command(language: &str, version: &str) {
    let config = Config::new();
    
    if let Some(installed) = config.get(language, version) {
        print_already_installed_message(language, version, installed.default);
        return;
    }

    let software = match get_language_version_safe(language, version).await {
        Ok(software) => software,
        Err(e) => {
            print_not_found_message(language, version, &e);
            return;
        }
    };
    
    install_software_with_data(software).await;
}

async fn get_language_version_safe(language: &str, version: &str) -> Result<api::Version, String> {
    // Try to get the language first to check if it exists
    let languages = api::get_languages().await;
    if !languages.iter().any(|l| l.to_lowercase() == language.to_lowercase()) {
        return Err(format!("Language '{}' not found", language));
    }
    
    let language_info = api::get_language(language).await;
    let version_exists = language_info.versions.iter()
        .any(|v| v.version == version);
    
    if !version_exists {
        return Err(format!("Version '{}' not found for language '{}'", version, language));
    }
    
    Ok(api::get_language_version(language, version).await)
}

fn print_already_installed_message(language: &str, version: &str, is_default: bool) {
    println!();
    print_box(&[("&eAlready Installed&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();
    if is_default {
        println!("{}", colorize(&format!("  &e{} {}&r is already installed and is the default version.", capitalize_first(language), version)));
    } else {
        println!("{}", colorize(&format!("  &e{} {}&r is already installed.", capitalize_first(language), version)));
        println!();
        let usage_line = format!(" &3pkit default {} {}&r  &8-&r  Set this version as default", language, version);
        print_box(
            &[(usage_line.as_str(), BoxAlignment::Left)],
            &BoxOptions {
                title: Some("Set as Default"),
                ..Default::default()
            },
        );
    }
    println!();
}

fn print_not_found_message(language: &str, version: &str, error: &str) {
    println!();
    print_box(&[("&cNot Found&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();
    println!("{}", colorize(&format!("  &e{} version {}&r was not found.", capitalize_first(language), version)));
    println!("{}", colorize(&format!("  &8Error: {}&r", error)));
    println!();
    let usage_line = format!(" &3pkit list {}&r  &8-&r  See available versions", language);
    print_box(
        &[(usage_line.as_str(), BoxAlignment::Left)],
        &BoxOptions {
            title: Some("Available Versions"),
            ..Default::default()
        },
    );
    println!();
}
