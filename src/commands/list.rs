
use std::fs;

use json;

use crate::filesystem;
use crate::parser::ClICommand;

pub fn main(command: &ClICommand) {

    println!("{:?}", command.command);

    let mut file: fs::File = filesystem::open("./test/config.json");
    let result: String = filesystem::read(&mut file);

    let json_data: json::JsonValue = json::parse(&result).unwrap();

    let node_enteries: json::JsonValue = json_data["node"].clone();

    for value in node_enteries.members() {
        let node: json::JsonValue = value.clone();
        let name: &str = node["name"].as_str().unwrap();
        let url: &str = node["version"].as_str().unwrap();
        println!("{}: {}", name, url);
    }
}