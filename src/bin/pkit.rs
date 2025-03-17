use std::env;

use pkit::parser;
// use pkit::request;

use pkit::commands::list;

// PATH="$(pwd):$PATH"

// async fn run() {
//     let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
//     request::download(url).await;
// }

// run().await;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_command: parser::ClICommand = parser::main(&args[2..]);

    match args[1].as_str() {
        "list" => {
            list::main(&cli_command);
        },
        _ => println!("Command not found"),
    }
    
}