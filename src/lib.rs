pub mod parser;

pub mod filesystem;

pub mod commands;

pub mod formatter;

pub mod api;

pub mod cli;

pub fn get_root_folder() -> String {
    // Get the path to the current executable
    let exe_path = std::env::current_exe().unwrap_or_else(|_| {
        eprintln!("Failed to get the current executable path. Falling back to HOME.");
        std::path::PathBuf::from(".")
    });

    // Get the parent directory of the executable
    let root_folder = exe_path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            eprintln!("Failed to determine the parent directory. Falling back to HOME.");
            std::env::var("HOME").unwrap_or_else(|_| String::from("."))
        });

    root_folder
}

pub fn formatted_path() -> String {
    let path = get_root_folder();

    // Replace the home part of path with $HOME
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
    let formatted_path = path.replace(&home, "$HOME");
    formatted_path
}