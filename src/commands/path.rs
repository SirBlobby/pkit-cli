use crate::filesystem::config::Config;
use crate::formatter::{
    print_message, MessageType,
    print_table_header, print_table_row, print_table_footer, colorize
};
use std::path::Path;

pub fn handle_path_command(action: &str, name: Option<&str>, path: Option<&str>) {
    let mut config = Config::new();

    match action {
        "add" => {
            if let (Some(name_str), Some(path_str)) = (name, path) {
                // Validate that the path exists
                if !Path::new(path_str).exists() {
                    print_message(MessageType::Error(&format!("Path '{}' does not exist", path_str)));
                    return;
                }
                
                config.add_path_source(name_str, path_str);
                config.write_env_script().expect("Failed to write environment script");
                print_message(MessageType::Success(&format!("Added path source '{}' at '{}'", name_str, path_str)));
            } else {
                print_message(MessageType::Error("Both name and path are required for adding a source"));
                println!("{}", colorize("&6Usage: &fpkit path add &e<name> <path>&r"));
            }
        }
        "set" => {
            if let Some(name_str) = name {
                // Check if source exists before setting
                if let Some(source) = config.get_path_source(name_str) {
                    if let Some(path_str) = path {
                        // Validate that the path exists
                        if !Path::new(path_str).exists() {
                            print_message(MessageType::Error(&format!("Path '{}' does not exist", path_str)));
                            return;
                        }
                        config.set_path_source(name_str, path_str);
                        config.write_env_script().expect("Failed to write environment script");
                        print_message(MessageType::Success(&format!("Updated path source '{}' to '{}'", name_str, path_str)));
                    } else {
                        print_message(MessageType::Info(&format!("Current path for '{}' is '{}'", name_str, source.path)));
                    }
                } else {
                    print_message(MessageType::Warning(&format!("Path source '{}' not found", name_str)));
                }
            } else {
                print_message(MessageType::Error("Name is required for setting a source"));
                println!("{}", colorize("&6Usage: &fpkit path set &e<name> &8[&e<path>&8]&r"));
            }
        }
        "remove" => {
            if let Some(name_str) = name {
                // Check if source exists before removing
                if config.get_path_source(name_str).is_some() {
                    config.remove_path_source(name_str);
                    config.write_env_script().expect("Failed to write environment script");
                    print_message(MessageType::Success(&format!("Removed path source '{}'", name_str)));
                } else {
                    print_message(MessageType::Warning(&format!("Path source '{}' not found", name_str)));
                }
            } else {
                print_message(MessageType::Error("Name is required for removing a source"));
                println!("{}", colorize("&6Usage: &fpkit path remove &e<name>&r"));
            }
        }
        "list" => {
            list_path_sources(&config);
        }
        _ => {
            print_message(MessageType::Error(&format!("Unknown action: '{}'. Use 'add', 'remove', or 'list'", action)));
            println!("{}", colorize("&6Usage:&r"));
            println!("{}", colorize("  &fpkit path add &e<name> <path>&8     &7Add a new path source&r"));
            println!("{}", colorize("  &fpkit path remove &e<name>&8         &7Remove a path source&r"));
            println!("{}", colorize("  &fpkit path list&8                  &7List all path sources&r"));
        }
    }
}

fn list_path_sources(config: &Config) {
    if config.sources.is_empty() {
        print_message(MessageType::Info("No path sources configured"));
        return;
    }

    println!("{}", colorize("&bPath Sources:&r"));
    println!();

    let columns = [("Name", 20), ("Path", 60)];
    print_table_header(&columns);

    for source in &config.sources {
        let path_str = shorten_path(&source.path, 60);
        let name_colored = format!("&a{}&r", source.name);
        let path_colored = format!("&e{}&r", path_str);
        let values = [name_colored.as_str(), path_colored.as_str()];
        print_table_row(&columns, &values);
    }

    print_table_footer(&columns);
}

/// Shortens a path string to fit within a max length, showing start and end, with ellipsis in the middle.
fn shorten_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        let ellipsis = "...";
        let keep = max_len.saturating_sub(ellipsis.len());
        let start_len = keep / 2;
        let end_len = keep - start_len;
        format!("{}{}{}", &path[..start_len], ellipsis, &path[path.len()-end_len..])
    }
}