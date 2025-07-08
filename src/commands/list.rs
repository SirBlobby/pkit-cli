use crate::{formatter::capitalize_first, formatter::print_colored};
use crate::api::{self};

use crate::filesystem::config::{Config, Installed};

pub async fn print_available_languages() {

    let mut languages:Vec<String>  = api::get_languages().await;

    for lang in languages.iter_mut() {
        *lang = capitalize_first(lang);
    }

    print_colored(&format!("&aAvailable Languages&r: &e{}", languages.join("&r, &e")));
    print_colored("Use &3pkit list <language>&r to list available versions for a specific language.");
}

pub async fn print_available_language_versions(language: &str) {
    let language: api::Language = api::get_language(language.to_lowercase().as_str()).await;

    print_colored(&format!("&aAvailable Versions for &e{}&r", capitalize_first(&language.name)));
    for version in language.versions.iter() {
        print_colored(&format!("&e{}&r (&3{}&r, &5{}&r)", version.version, version.platform, version.arch));
    }
    println!();
    print_colored(&format!("Use &3pkit install {} <version>&r to install a specific version.", language.name));
}

pub fn print_installed_languages() {
    print_colored("&aInstalled Languages&r:");
    let config = Config::new();
    let installed: Vec<Installed> = config.installed;

    for lang in installed.iter() {
        print_colored(&format!("&e{}&r (&3{}&r)", lang.language, lang.version));
    }
    println!();
    print_colored("Use &3pkit default <language>&r to set a default language.");
}

pub async fn handle_list_command(language: Option<&String>, installed: bool) {
    if installed {
        print_installed_languages();
    } else if let Some(lang) = language {
        print_available_language_versions(lang).await;
    } else {
        print_available_languages().await;
    }
}
