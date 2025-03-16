use pkit::parser;

mod helper;

fn main() {

    let command = "ls -l -a";
    parser::string_reader(command);

    helper::main();

}