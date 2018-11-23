extern crate regex;
//Starte UmL Parser
use crate::parser::klassendiagramm::klassendiagramm::Klassendiagramm;
use crate::parser::klassendiagramm::klassendiagramm;
use crate::parser::vektor;
use self::regex::Regex;
static mut OBJECTCOUNTER: u32 = 0;

pub unsafe fn count_all_objects() {
    OBJECTCOUNTER = OBJECTCOUNTER + 1;
}

pub unsafe fn get_counter() -> u32 {
    OBJECTCOUNTER
}

pub fn sammle_alle_diagramme(){

}
struct Diagramme{
    klassendiagramme: Vec<Klassendiagramm>
}
impl Diagramme{
    pub fn get_klassendiagramme(&self)->&Vec<Klassendiagramm>{
        &self.klassendiagramme
    }
}
pub fn starte_umlparser(input: &String) {
    unsafe {
        OBJECTCOUNTER = 0;
    }
    let klassendiagramme =klassendiagramm::parse_klassendiagramme(&input);
    let diagramme = Diagramme{klassendiagramme};
    parse_aktivitaetendiagramm(&input);
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
