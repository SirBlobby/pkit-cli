use std::env;
use std::process::Command;

use crate::{formatted_path, get_root_folder};

use crate::parser::ClICommand;

use crate::filesystem::config::{Config, Installed};

use crate::formatter::colorize;

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
        println!("{} version {} is not installed.", lang, version);
        std::process::exit(1);
    }
}

fn set_default(cli: &ClICommand) {
    let lang = &cli.command[0];
    let version = &cli.command[1];

    let installed = get_installed_language(lang, version);

    let mut config = Config::new();
    config.set_default(&installed.language, &installed.version);

    add_pkit_path(&installed.path, &installed.language);

    println!("{}", colorize(&format!("{} version {} is now the default.", installed.language, installed.version)));
}


pub fn main(cli: &ClICommand) {
    match cli.command.len() {
        2 => {
            set_default(cli);
        },
        _ => {
            println!("{}", colorize(&format!("Invalid number of arguments. Expected 2, got {}", cli.command.len())));
            std::process::exit(1);
        }
    }
}
