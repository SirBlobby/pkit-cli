use crate::filesystem::config::{Config, Installed};

use crate::formatter::{print_colored, print_error, print_success};

#[warn(unused_variables)]
#[cfg(target_os = "linux")]
pub fn add_pkit_path(_path: &str, _lang: &str) {
    return;
}

// pub fn update_pkit_path(new_path: &str, lang: &str) {}

// pub fn remove_pkit_path() {}

fn get_installed_language(lang: &str, version: &str) -> Installed {
    let config = Config::new();

    let installed: Option<&Installed> = config.get(lang, version);

    if installed.is_some() {
        Installed { language: installed.unwrap().language.clone(), version: installed.unwrap().version.clone(), path: installed.unwrap().path.clone(), default: installed.unwrap().default }
    } else {
        print_error(&format!("{} version {} is not installed.", lang, version));
        std::process::exit(1);
    }
}

pub fn run_default(language: &str) {
    // For now, let's assume version can be derived from installed packages
    // In a real implementation, you might want to list available versions
    // or use the latest installed version
    let config = Config::new();
    let installed_versions: Vec<&Installed> = config.installed.iter()
        .filter(|pkg| pkg.language == language)
        .collect();
    
    if installed_versions.is_empty() {
        print_error(&format!("No versions of {} are installed.", language));
        std::process::exit(1);
    }
    
    if installed_versions.len() == 1 {
        let installed = installed_versions[0];
        let mut config = Config::new();
        config.set_default(&installed.language, &installed.version);
        add_pkit_path(&installed.path, &installed.language);
        print_success(&format!("{} version {} is now the default.", installed.language, installed.version));
    } else {
        print_colored(&format!("&eMultiple versions of {} are installed:&r", language));
        for (i, installed) in installed_versions.iter().enumerate() {
            print_colored(&format!("  &3{}&r: &e{}&r", i + 1, installed.version));
        }
        print_colored(&format!("Please specify a version: &3pkit default {} <version>&r", language));
    }
}

pub fn run_default_with_version(language: &str, version: &str) {
    let installed = get_installed_language(language, version);
    let mut config = Config::new();
    config.set_default(&installed.language, &installed.version);
    add_pkit_path(&installed.path, &installed.language);
    print_success(&format!("{} version {} is now the default.", installed.language, installed.version));
}

pub fn handle_default_command(language: &str, version: Option<&String>) {
    if let Some(ver) = version {
        run_default_with_version(language, ver);
    } else {
        run_default(language);
    }
}
