extern crate regex;
//Starte UmL Parser
use crate::parser::klassendiagramm::klassendiagramm::Klassendiagramm;
use crate::parser::klassendiagramm::klassendiagramm;
use self::regex::Regex;

static mut OBJECTCOUNTER: usize = 0;

///inkrementiert den Zähler
pub unsafe fn count_all_objects() {
    OBJECTCOUNTER = OBJECTCOUNTER + 1;
}
///Zählt die Anzahl aller sichtbar erzeugten Objekte im Uml Diagramm
pub unsafe fn get_counter() -> usize {
    OBJECTCOUNTER
}


///Hält alle Klassendiagramme
pub struct Diagramme{
    klassendiagramme: Vec<Klassendiagramm>
}
impl Diagramme{
    ///gibt einen Vektor aller Klassendiagramme zurück
    pub fn get_klassendiagramme(self)->Vec<Klassendiagramm>{
        self.klassendiagramme
    }
}
///startet die Oberste Ebene des Parsers und ruft alle drunter liegenden Parser Methoden auf.
pub fn starte_umlparser<'a>(input: &String)->Diagramme {
    unsafe {
        OBJECTCOUNTER = 0;
    }
    let klassendiagramme =klassendiagramm::parse_klassendiagramme(&input);
    let diagramme = Diagramme{klassendiagramme};
    diagramme
}

///Untersucht den Text mittels regulären Ausdruck und liefert die Ergebnisse in einem Vektor zurück
pub fn parse_text_tovector(text: &String, re: &Regex) -> Vec<String> {
    let mut v: Vec<String> = vec![];
    for caps in re.captures_iter(&text) {
        let text = caps.get(1).unwrap().as_str();
        v.push(text.to_string());
    }
    v
}
///Untersucht den Text mittels regulären Ausdruck und liefert die Ergebnisse in einem String
pub fn parse_text_to_string(text: &String, re: &Regex) -> String {
    let v: Vec<String> = parse_text_tovector(&text, &re);
    if v.len() == 0 {
        return "".to_string();
    } else {
        let k: &str = v.get(0).unwrap();
        return k.to_string();
    }
}