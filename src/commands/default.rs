use crate::filesystem::config::Config;
use crate::formatter::{
    capitalize_first, colorize, print_box, print_table_footer, print_table_header,
    print_table_row, BoxAlignment, BoxOptions,
};

pub fn handle_default_command(language: &str, version: Option<&String>, show: bool) {
    let mut config = Config::new();

    if show {
        if let Some(default) = config.get_default(language) {
            println!("{}", colorize(&format!("  &eDefault {} version: {}&r", language, default.version)));
        } else {
            println!("{}", colorize(&format!("  &eNo default version set for {}&r", language)));
        }
        return;
    }

    if let Some(ver) = version {
        if config.get(language, ver).is_some() {
            config.set_default(language, ver);
            config.write_env_script().expect("Failed to write environment script");
            print_success_message(language, ver);
        } else {
            print_not_installed_message(language, ver);
            std::process::exit(1);
        }
    } else {
        let installed_versions: Vec<_> = config.installed.iter()
            .filter(|pkg| pkg.language == language)
            .cloned()
            .collect();

        if installed_versions.is_empty() {
            print_no_versions_installed_message(language);
        } else if installed_versions.len() == 1 {
            let installed = &installed_versions[0];
            config.set_default(language, &installed.version);
            config.write_env_script().expect("Failed to write environment script");
            print_success_message(language, &installed.version);
        } else {
            print_multiple_versions_installed_message(language, &installed_versions);
        }
    }
}

fn print_success_message(language: &str, version: &str) {
    println!();
    print_box(&[("&aSuccess&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();
    println!("{}", colorize(&format!("  &e{} {}&r is now the default version.", capitalize_first(language), version)));
    println!();
}

fn print_not_installed_message(language: &str, version: &str) {
    println!();
    print_box(&[("&cError&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();
    println!("{}", colorize(&format!("  &e{} version {}&r is not installed.", capitalize_first(language), version)));
    println!();
    let usage_line = format!(" &3pkit install {} {}&r  &8-&r  Install this version", language, version);
    print_box(
        &[(usage_line.as_str(), BoxAlignment::Left)],
        &BoxOptions {
            title: Some("Installation"),
            ..Default::default()
        },
    );
    println!();
}

fn print_no_versions_installed_message(language: &str) {
    println!();
    let title = format!("&eNo versions of {} are installed.&r", capitalize_first(language));
    print_box(&[(title.as_str(), BoxAlignment::Center)], &BoxOptions::default());
    println!();
    let usage_line = format!(" &3pkit install {} <version>&r  &8-&r  Install a specific version", language);
    print_box(
        &[(usage_line.as_str(), BoxAlignment::Left)],
        &BoxOptions {
            title: Some("Installation"),
            ..Default::default()
        },
    );
    println!();
}

fn print_multiple_versions_installed_message(language: &str, installed_versions: &[crate::filesystem::config::Installed]) {
    println!();
    let title = format!("&eMultiple versions of {} are installed&r", capitalize_first(language));
    print_box(&[(title.as_str(), BoxAlignment::Center)], &BoxOptions::default());
    println!();

    let columns = [("Version", 20), ("Status", 17)];
    print_table_header(&columns);

    for installed in installed_versions.iter() {
        let version_str = format!("&e{}&r", installed.version);
        let status_str = if installed.default { "&aDefault&r".to_string() } else { "&8Available&r".to_string() };
        let values = [version_str.as_str(), status_str.as_str()];
        print_table_row(&columns, &values);
    }
    print_table_footer(&columns);
    println!();
    let usage_line = format!(" &3pkit default {} <version>&r  &8-&r  Set a specific version as default", language);
    print_box(
        &[(usage_line.as_str(), BoxAlignment::Left)],
        &BoxOptions {
            title: Some("Usage"),
            ..Default::default()
        },
    );
    println!();
}