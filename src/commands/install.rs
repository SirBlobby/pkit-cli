use crate::{
    api::{self, request},
    filesystem::{self, config::Config, get_pkit_dir},
    formatter::{capitalize_first, colorize, print_box, BoxAlignment, BoxOptions},
};
use std::path::PathBuf;

pub async fn install_software(language: &str, version: &str) {
    let software = api::get_language_version(language, version).await;

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
        let usage_line = format!(" &3pkit default {} {}&r  &8-&r  Set this version as default later", language, version);
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
    install_software(language, version).await;
}
