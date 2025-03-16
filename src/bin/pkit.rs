use std::env;

use pkit::parser;
use pkit::request;

// PATH="$(pwd):$PATH"

async fn run() {
    let url = "https://www.dropbox.com/scl/fi/qmny11w7wkjnskyh7g76m/Cobblemon-Official-Fabric-1.0.0.mrpack?rlkey=s3nkxs0u18s123a4pylvmphzn&st=36q91qxa&dl=1";
    request::download(url).await;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = parser::main(&args[1..]);

    println!("{:?}", cli.command);

    for flag in cli.flags {
        println!("{:?} - {:?}", flag.flag, flag.value);
    }

    run().await;
}