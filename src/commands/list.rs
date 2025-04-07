use crate::{formatter::colorize, formatter::capitalize_first};
use crate::api::{self};

use crate::parser::ClICommand;

use crate::filesystem::config::{Config, Installed};

async fn print_available_languages() {

    let mut languages:Vec<String>  = api::get_languages().await;

    for lang in languages.iter_mut() {
        *lang = capitalize_first(lang);
    }

    println!("{}", colorize(&format!("&aAvailable Languages&r: &e{}", languages.join("&r, &e"))));
    println!("{}", colorize("Use &3pkit list <language>&r to list available versions for a specific language."));
}

async fn print_available_language_versions(language: &str) {
    let language: api::Language = api::get_language(language.to_lowercase().as_str()).await;

    println!("{}", colorize(&format!("&aAvailable Versions for &e{}&r", capitalize_first(&language.name))));
    for version in language.versions.iter() {
        println!("{}", colorize(&format!("&e{}&r (&3{}&r, &5{}&r)", version.version, version.platform, version.arch)));
    }
    println!();
    println!("{}", colorize(&format!("Use &3pkit install {} <version>&r to install a specific version.", language.name)));
}

fn print_installed_languages() {
    println!("{}", colorize("&aInstalled Languages&r:"));
    let config = Config::new();
    let installed: Vec<Installed> = config.installed;

    for lang in installed.iter() {
        println!("{}", colorize(&format!("&e{}&r (&3{}&r)", lang.language, lang.version)));
    }
    println!();
    println!("{}", colorize("Use &3pkit default <language>&r to set a default language."));
}


// pkit list - List available languages
// pkit list <language> - List available versions for a specific language
// pkit list <language> <version> - List available platforms for a specific version
// pkit list installed - List installed languages

pub async fn main(cli: &ClICommand) {

    if !cli.command.is_empty() {

        match cli.get_first().as_str() {
            "installed" => {
                print_installed_languages();
            },
            _ => {
                print_available_language_versions(&cli.get_first()).await;
            }
        }

    } else {
        print_available_languages().await;
    }

}