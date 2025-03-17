use crate::{formatter::colorize, formatter::capitalize_first};
use crate::api;

use crate::parser::ClICommand;

async fn print_available_languages() {

    let mut languages:Vec<String>  = api::get_languages().await;

    for lang in languages.iter_mut() {
        *lang = capitalize_first(lang);
    }

    println!("{}", colorize(&format!("&aAvailable Languages&r: &e{}", languages.join("&r, &e"))));
    println!("{}", colorize(&format!("Use &3pkit list <language>&r to list available versions for a specific language.")));
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


// pkit list - List available languages
// pkit list <language> - List available versions for a specific language
// pkit list <language> <version> - List available platforms for a specific version


pub async fn main(cli: &ClICommand) {

    if cli.command.len() > 0 {
        print_available_language_versions(&cli.get_first()).await;
    } else {
        print_available_languages().await;
    }

}