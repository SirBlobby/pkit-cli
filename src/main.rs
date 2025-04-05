use std::env;

use pkit::filesystem::config::Config;
use pkit::parser;
// use pkit::request;

use pkit::commands::{list, install, default};

// PATH="$(pwd):$PATH"

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let _ = Config::new();

    if args.len() > 1 {

        let cli_command: parser::ClICommand = parser::main(&args[2..]);

        match args[1].as_str() {
            "list" => {
                list::main(&cli_command).await;
            },
            "default" => {
                default::main(&cli_command);
            },
            "install" => {
                install::main(&cli_command).await;
            },
            _ => println!("Command not found"),
        }
    } else {
        println!("Usage: pkit <command> <subcommand> [args]");
        return;
    }
    
}