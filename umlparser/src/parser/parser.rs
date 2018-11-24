extern crate regex;
//Starte UmL Parser
use crate::parser::klassendiagramm::klassendiagramm::Klassendiagramm;
use crate::parser::klassendiagramm::klassendiagramm;
use crate::parser::relation;
use crate::layout::graphbuilder;
use self::regex::Regex;
static mut OBJECTCOUNTER: usize = 0;

pub unsafe fn count_all_objects() {
    OBJECTCOUNTER = OBJECTCOUNTER + 1;
}

pub unsafe fn get_counter() -> usize {
    OBJECTCOUNTER
}

pub fn sammle_alle_diagramme(){

}
pub struct Diagramme{
    klassendiagramme: Vec<Klassendiagramm>
}
impl Diagramme{
    pub fn get_klassendiagramme(self)->Vec<Klassendiagramm>{
        self.klassendiagramme
    }
}
pub fn starte_umlparser(input: &String)->Diagramme {
    unsafe {
        OBJECTCOUNTER = 0;
    }
    let klassendiagramme =klassendiagramm::parse_klassendiagramme(&input);
    let mut diagramme = Diagramme{klassendiagramme};
    parse_aktivitaetendiagramm(&input);
    diagramme
}




pub fn parse_aktivitaetendiagramm(input: &String) {}


pub fn parse_text_tovector(text: &String, re: &Regex) -> Vec<String> {
    let mut v: Vec<String> = vec![];
    for caps in re.captures_iter(&text) {
        let text = caps.get(1).unwrap().as_str();
        v.push(text.to_string());
    }
    v
}
pub fn parse_text_to_string(text: &String, re: &Regex) -> String {
    let mut v: Vec<String> = parse_text_tovector(&text, &re);
    if v.len() == 0 {
        return "".to_string();
    } else {
        let k: &str = v.get(0).unwrap();
        return k.to_string();
    }
}
