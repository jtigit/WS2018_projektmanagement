extern crate regex;

use std::fs;
use regex::Regex;
//Starte UmL Parser

use crate::umlparser::klassendiagramm::klasse;
use crate::umlparser::vektor;

static mut Objectcounter: u32 =0;

    pub unsafe fn count_all_objects(){
        Objectcounter = Objectcounter + 1;
    }
    pub unsafe fn get_counter() ->u32{
        Objectcounter
    }
    pub fn starte_umlparser(input: &String) {
        unsafe {
            Objectcounter = 0;
        }
        parse_klassendiagramm(&input);
        parse_aktivitaetendiagramm(&input);
    }

    pub fn parse_klassendiagramm(input: &String) {
        klasse::baue_klassen(input,);
        vektor::baue_vektoren(input);
    }

    pub fn parse_aktivitaetendiagramm(input: &String) {}


pub fn parse_text(text: &String, re: &Regex) -> Vec<String> {
    let mut v: Vec<String> = vec![];

    for caps in re.captures_iter(&text) {
        let text = caps.get(1).unwrap().as_str();
        v.push(text.to_string());
        println!("Element: {:?}", &caps["text"]);
    }
    v
}
