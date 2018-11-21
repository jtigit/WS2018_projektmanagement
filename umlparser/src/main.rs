
extern crate regex;

use std::fs;
use regex::Regex;

mod umlparser;


fn main() {
    umlparser::parser::starte_umlparser(&readFile());
}

pub fn readFile() -> String {
    let filename = String::from("example.txt");
    let contents = fs::read_to_string(&filename);
    let text = contents.unwrap();
    text
}