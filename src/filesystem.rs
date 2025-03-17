use std::fs::File;
use std::io::prelude::*;

pub fn open(path: &str) -> std::fs::File {
    let file = std::fs::File::open(path).unwrap();
    file
}

pub fn read(file: &mut std::fs::File) -> String {
    let mut contents = String::new();
    let _ = file.read_to_string( &mut contents);
    contents
}

pub fn write(path: &str, contents: &str) -> bool {
    let mut file = File::create(path).unwrap();
    let _ = file.write_all(contents.as_bytes());
    true
}

pub fn append(path: &str, contents: &str) -> bool {
    let mut file = File
        ::open(path)
        .expect("Unable to open file");
    let _ = file.write_all(contents.as_bytes());
    true
}