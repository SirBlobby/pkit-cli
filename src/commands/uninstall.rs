use crate::{
    filesystem::{config::Config, get_pkit_dir},
    formatter::{capitalize_first, colorize, print_box, BoxAlignment, BoxOptions},
};
use std::fs;

pub fn handle_uninstall_command(language: &str, version: Option<&String>, all: bool) {
    let mut config = Config::new();

    if all {
        uninstall_all_versions(language, &mut config);
    } else if let Some(ver) = version {
        uninstall_specific_version(language, ver, &mut config);
    } else {
        print_usage_message(language);
    }
}

fn uninstall_specific_version(language: &str, version: &str, config: &mut Config) {
    if let Some(installed) = config.get(language, version) {
        let was_default = installed.default;
        
        // Remove from config
        config.remove_install(language, version);
        
        // Remove the actual installation directory
        let pkit_dir = get_pkit_dir().expect("Failed to get pkit directory");
        let version_dir = pkit_dir
            .join("bin")
            .join(language)
            .join(version);
            
        if version_dir.exists() {
            if let Err(e) = fs::remove_dir_all(&version_dir) {
                eprintln!("Warning: Failed to remove directory {:?}: {}", version_dir, e);
            }
        }
        
        // If this was the default version, check if there are other versions and prompt to set a new default
        if was_default {
            handle_default_removal(language, config);
        }
        
        // Update environment script
        config.write_env_script().expect("Failed to write environment script");
        
        print_success_message(language, version);
    } else {
        print_not_installed_message(language, version);
    }
}

fn uninstall_all_versions(language: &str, config: &mut Config) {
    let installed_versions: Vec<_> = config.installed.iter()
        .filter(|pkg| pkg.language == language)
        .cloned()
        .collect();
    
    if installed_versions.is_empty() {
        print_no_versions_installed_message(language);
        return;
    }
    
    // Confirm uninstall all
    println!();
    println!("{}", colorize(&format!("  &eThis will uninstall all {} versions. Are you sure? (y/n): &r", language)));
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("{}", colorize("  &eUninstall cancelled.&r"));
        return;
    }
    
    // Remove all versions from config
    for installed in &installed_versions {
        config.remove_install(language, &installed.version);
    }
    
    // Remove the entire language directory
    let pkit_dir = get_pkit_dir().expect("Failed to get pkit directory");
    let language_dir = pkit_dir.join("bin").join(language);
    
    if language_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&language_dir) {
            eprintln!("Warning: Failed to remove directory {:?}: {}", language_dir, e);
        }
    }
    
    // Update environment script
    config.write_env_script().expect("Failed to write environment script");
    
    print_all_versions_removed_message(language, &installed_versions);
}

fn handle_default_removal(language: &str, config: &mut Config) {
    let remaining_versions: Vec<_> = config.installed.iter()
        .filter(|pkg| pkg.language == language)
        .cloned()
        .collect();
    
    if remaining_versions.is_empty() {
        println!("{}", colorize(&format!("  &eNo more {} versions installed.&r", language)));
    } else if remaining_versions.len() == 1 {
        // Automatically set the only remaining version as default
        let remaining = &remaining_versions[0];
        config.set_default(language, &remaining.version);
        println!("{}", colorize(&format!("  &e{} {}&r is now the default version.", capitalize_first(language), remaining.version)));
    } else {
        // Multiple versions remain, ask user to choose a new default
        println!();
        println!("{}", colorize(&format!("  &eThe default {} version was removed. Available versions:&r", language)));
        for (i, installed) in remaining_versions.iter().enumerate() {
            println!("  {}. {}", i + 1, installed.version);
        }
        println!();
        println!("{}", colorize("  &eEnter the number of the version to set as default (or press Enter to skip): &r"));
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if !input.is_empty() {
            if let Ok(choice) = input.parse::<usize>() {
                if choice > 0 && choice <= remaining_versions.len() {
                    let selected = &remaining_versions[choice - 1];
                    config.set_default(language, &selected.version);
                    println!("{}", colorize(&format!("  &e{} {}&r is now the default version.", capitalize_first(language), selected.version)));
                } else {
                    println!("{}", colorize("  &eInvalid choice. No default set.&r"));
                }
            } else {
                println!("{}", colorize("  &eInvalid input. No default set.&r"));
            }
        } else {
            println!("{}", colorize("  &eNo default set.&r"));
        }
    }
}

fn print_success_message(language: &str, version: &str) {
    println!();
    print_box(&[("&aUninstall Complete&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();
    println!("{}", colorize(&format!("  &e{} {}&r has been successfully uninstalled.", capitalize_first(language), version)));
    println!();
}

fn print_all_versions_removed_message(language: &str, versions: &[crate::filesystem::config::Installed]) {
    println!();
    print_box(&[("&aUninstall Complete&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();
    println!("{}", colorize(&format!("  &eAll {} versions have been uninstalled:&r", capitalize_first(language))));
    for version in versions {
        println!("  - {}", version.version);
    }
    println!();
}

fn print_not_installed_message(language: &str, version: &str) {
    println!();
    print_box(&[("&cError&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();
    println!("{}", colorize(&format!("  &e{} version {}&r is not installed.", capitalize_first(language), version)));
    println!();
    let usage_line = format!(" &3pkit list {} --installed&r  &8-&r  Show installed versions", language);
    print_box(
        &[(usage_line.as_str(), BoxAlignment::Left)],
        &BoxOptions {
            title: Some("Check Installed"),
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
}

fn print_usage_message(language: &str) {
    println!();
    let title = format!("&eUninstall {} Version&r", capitalize_first(language));
    print_box(&[(title.as_str(), BoxAlignment::Center)], &BoxOptions::default());
    println!();
    
    let usage_lines = [
        format!(" &3pkit uninstall {} <version>&r  &8-&r  Uninstall a specific version", language),
        format!(" &3pkit uninstall {} --all&r     &8-&r  Uninstall all versions", language),
    ];
    
    for line in &usage_lines {
        print_box(
            &[(line.as_str(), BoxAlignment::Left)],
            &BoxOptions {
                title: if line == &usage_lines[0] { Some("Usage") } else { None },
                ..Default::default()
            },
        );
    }
    println!();
}
