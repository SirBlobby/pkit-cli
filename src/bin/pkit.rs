use std::env;

use pkit::parser;
// use pkit::request;

use pkit::commands::{list, install};

// PATH="$(pwd):$PATH"

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {

        let cli_command: parser::ClICommand = parser::main(&args[2..]);

        match args[1].as_str() {
            "list" => {
                list::main(&cli_command).await;
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