use std::env;

use pkit::parser;

// PATH="$(pwd):$PATH"

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = parser::main(&args[1..]);

    println!("{:?}", cli.command);

    for flag in cli.flags {
        println!("{:?} - {:?}", flag.flag, flag.value);
    }
}