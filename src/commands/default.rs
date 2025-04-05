use std::env;
use std::process::Command;

use crate::filesystem;
use crate::parser::ClICommand;

use crate::filesystem::config::{Config, Installed};

use crate::formatter::colorize;

// if linux or macos, check if the path to the pkit-init.sh file in bin folder is in the PATH
// if linux or macos, add the path to the pkit-init.sh file in bin folder
// if windows, add directly to the PATH

// add this to the path for linux and macos

// export "$HOME/{pkit_dir}"
// [[ -s "$HOME/{pkit_dir}/bin/pkit-init.sh" ]] && source "$HOME/{pkit_dir}/bin/pkit-init.sh"


#[cfg(target_os = "linux")]
pub fn add_pkit_path(new_path: &str, lang: &str) {
    let shell = env::var("SHELL").unwrap_or_default();
    let shell_config = if shell.contains("zsh") {
        "~/.zshrc"
    } else if shell.contains("bash") {
        "~/.bashrc"
    } else {
        "~/.profile"
    };

    // check if the pkit-init.sh file is in the PATH already
    let output = Command::new("sh")
        .args(&["-c", "command -v pkit-init.sh"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Path already updated on Linux.");
        return;
    }

    // add the path to the ./bin/pkit-init.sh file in the bin folder use export "{path}:$PATH"
    let labeled_path = format!("export \"{}\" # pkit_{}", new_path, lang);
    filesystem::append("./bin/pkit-init.sh", &labeled_path);

    if output.status.success() {
        println!("Path updated successfully on Linux with a 'pkit' label.");
        println!("Don't forget to reload your shell configuration file with 'source {}' or restart your terminal.", shell_config);
    } else {
        println!("Failed to update path on Linux. Error: {:?}", output.stderr);
    }

}

#[cfg(target_os = "macos")]
pub fn add_pkit_path(new_path: &str, lang: &str) {
    let shell = env::var("SHELL").unwrap_or_default();
    let shell_config = if shell.contains("zsh") {
        "~/.zshrc"
    } else if shell.contains("bash") {
        "~/.bashrc"
    } else {
        "~/.profile"
    };

    // check if the pkit-init.sh file is in the PATH already
    let output = Command::new("sh")
        .args(&["-c", "command -v pkit-init.sh"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Path already updated on macOS.");
        return;
    }

    // add the path to the ./bin/pkit-init.sh file in the bin folder use export "{path}:$PATH"
    let labeled_path = format!("export \"{}\" # pkit_{}", new_path, lang);
    filesystem::append("./bin/pkit-init.sh", &labeled_path);

    if output.status.success() {
        println!("Path updated successfully on macOS with a 'pkit' label.");
        println!("Don't forget to reload your shell configuration file with 'source {}' or restart your terminal.", shell_config);
    } else {
        println!("Failed to update path on macOS. Error: {:?}", output.stderr);
    }
}

#[cfg(target_os = "windows")]
pub fn add_pkit_path(new_path: &str, lang: &str) {
    return;
}


pub fn update_pkit_path(new_path: &str, lang: &str) {
    #[cfg(target_os = "windows")]
    {
        let labeled_path = format!("{};{}", new_path, "pkit_marker");
        let output = Command::new("cmd")
            .args(&["/C", &format!("setx PATH \"{};{}\"", labeled_path, env::var("PATH").unwrap_or_default())])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("Path updated successfully on Windows with a 'pkit' marker.");
        } else {
            println!("Failed to update path on Windows. Error: {:?}", output.stderr);
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let shell = env::var("SHELL").unwrap_or_default();
        let shell_config = if shell.contains("zsh") {
            "~/.zshrc"
        } else if shell.contains("bash") {
            "~/.bashrc"
        } else {
            "~/.profile"
        };

        let labeled_path = format!("export PKIT_PATH=\"{}\" # pkit_{}", new_path, lang);
        let command = format!("echo '{}' >> {}", labeled_path, shell_config);

        let output = Command::new("sh")
            .args(&["-c", &command])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("Path updated successfully on Linux/macOS with a 'pkit' label.");
            println!("Don't forget to reload your shell configuration file with 'source {}' or restart your terminal.", shell_config);
        } else {
            println!("Failed to update path on Linux/macOS. Error: {:?}", output.stderr);
        }
    }
}


pub fn remove_pkit_path() {}

fn get_installed_language(lang: &str, version: &str) -> Installed {
    let config = Config::new();

    let installed: Option<&Installed> = config.get(&lang, &version);

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
