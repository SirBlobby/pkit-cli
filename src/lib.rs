

pub mod parser;

pub mod filesystem;

pub mod commands;

pub mod formatter;

pub mod api;

pub fn get_root_folder() -> String {
    let root_path = std::env::current_exe().unwrap();
    let parent_path = root_path.parent().unwrap();

    parent_path.display().to_string()
}