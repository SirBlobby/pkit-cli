use pkit::parser;

fn main() {

    let command = "ls -l -a";
    let pas = parser::main(command);

    println!("{:?}", pas.command);
}