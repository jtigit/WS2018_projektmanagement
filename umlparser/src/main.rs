extern crate regex;

use std::fs;
use regex::Regex;
mod umlparser;

fn main() {

    umlparser::parser::starte_umlparser(&read_file());
}

pub fn read_file() -> String {
    let filename = String::from("example.txt");
    let contents = fs::read_to_string(&filename);
    let text = contents.unwrap();
    text
}
