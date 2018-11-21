
extern crate regex;

use std::fs;
use regex::Regex;

mod klassendiagramm;

fn main() {
    klassendiagramm::klasse::baue_klassen();
}