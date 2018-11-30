extern crate regex;

use self::regex::Regex;
use crate::parser::parser;
use crate::parser::klassendiagramm::klassendiagramm;
use parser::klassendiagramm::klassendiagramm::Klasse;

#[derive(Clone)]
pub struct Relation {
    //Können mehrere Punkte sein. erster und letzer sind start und end
    // 0: A = Startknoten
    // 1: D = EndKnoten         r2
    // 2: B = Zwischenknoten    A-------B
    // 3: C = Zwischenknoten    m2      | desc
    // 4: m1 = Beschriftung             |         r1
    // 5: m2 = Beschriftung             C-------->D
    // 6: r1 = Beschriftung                        m1
    // 7: r2 = Beschriftung
    // 8: desc = Beschriftung
    pub koord: Vec<(u32, u32)>,
    pub richtung: u32,

    verbindet: (String, String),
    // 0: typ
    // 1: m1 = Beschriftung             |         r1
    // 2: m2 = Beschriftung             C-------->D
    // 3: r1 = Beschriftung                        m1
    // 4: r2 = Beschriftung
    // 5: desc = Beschriftung
    beschreibung: Vec<String>,
    pk: usize,
    start_klasse: Klasse,
    end_klasse: Klasse,
    //Knotenpunkte zum Zeichnen von A nach D : A,B,C,D
}

//Standard Konstruktor
pub fn build_relation(startknoten: String, endknoten: String, beschreibung: Vec<String>,
                      start_klasse: Klasse, end_klasse: Klasse) -> Relation {
    let a: (u32, u32) = (0, 0);
    let richtung: u32 = 0;
    let mut koord: Vec<(u32, u32)> = vec![];
    koord.push(a);
    koord.push(a);
    let pk;// primary key
    unsafe {
        parser::count_all_objects();
        pk = parser::get_counter();
    }
    Relation { koord, verbindet: (startknoten, endknoten), richtung, beschreibung, pk, start_klasse, end_klasse }
}

impl Relation {
    pub fn get_startklasse(&self) -> &Klasse {
        &self.start_klasse
    }
    pub fn get_endklasse(&self) -> &Klasse {
        &self.end_klasse
    }
    pub fn get_x_startknoten(&self) -> &u32 {
        let (x, _y) = self.koord.first().unwrap();
        x
    }
    pub fn get_y_startknoten(&self) -> &u32 {
        let (_x, y) = self.koord.first().unwrap();
        y
    }
    pub fn get_x_endknoten(&self) -> &u32 {
        let (x, _y) = self.koord.get(1).unwrap();
        x
    }
    pub fn get_y_endknoten(&self) -> &u32 {
        let (_x, y) = self.koord.get(1).unwrap();
        y
    }
    pub fn set_startknoten(&mut self, x: u32, y: u32) {
        self.koord[0] = (x, y);
    }

    pub fn set_endknoten(&mut self, x: u32, y: u32) {
        self.koord[1] = (x, y);
    }
    pub fn get_name_startknoten(&self) -> &String {
        let (x, _y) = &self.verbindet;
        &x
    }
    pub fn get_name_endknoten(&self) -> &String {
        let (_x, y) = &self.verbindet;
        &y
    }
    pub fn get_koord(&self) -> &Vec<(u32, u32)> { &self.koord }
    pub fn get_typ(&self) -> &str {
        &self.beschreibung.get(0).unwrap()
    }
    pub fn get_beschr_startknoten(&self) -> &String {
        &self.beschreibung.get(1).unwrap()
    }
    pub fn get_beschr_endknoten(&self) -> &String {
        &self.beschreibung.get(2).unwrap()
    }
    //Gibt die Richtung eines Punktes(x2,y2) relativ zum Punkt (x1,y1) zurück
    // Für die Berechnung wird das normale Kartesensystem verwendet!!
    // das Ergebniss entspricht aber eines wo die y achse nach unten verläuft .
    // das heisst (0,1) ist "Oben" zu (0,0) im Ergebniss.
    pub fn bestimme_richtung(x1: i32, y1: i32, x2: i32, y2: i32) -> String {
        let mut value: String = "".to_string();
        let dx: i32 = x2 - x1;
        let dy: i32 = y2 - y1;
        let g = dx - dy;
        let h = -dx - dy;
        if g <= 0 && h < 0 {
            value = "Unten".to_string();
        } else if g > 0 && h <= 0 {
            value = "Rechts".to_string();
        } else if g >= 0 && h > 0 {
            value = "Oben".to_string();
        } else if g < 0 && h >= 0 {
            value = "Links".to_string();
        }
        value
    }
    //Setzt die ZwischenPunkte und die der Beschriftungen fest.
    pub fn setze_koordinaten(& mut self,ax:u32,ay:u32,dx:u32,dy:u32,richtung:String){
        let mut bx:u32=0; let mut by:u32=0;
        let mut cx:u32=0; let mut cy:u32=0;
        let mut m1x:u32=0; let mut m1y:u32=0;
        let mut m2x:u32=0; let mut m2y:u32=0;
        let mut r1x:u32=0; let mut r1y:u32=0;
        let mut r2x:u32=0; let mut r2y:u32=0;
        let mut descx:u32=0; let mut descy:u32=0;
        if richtung == "Unten"{
            bx=ax; by = dy-ay /2;
            cx=dx; cy=by;
        }else if richtung == "Oben"{
            bx=ax; by = ay-dy /2;
            cx=dx; cy=by;
        }else if richtung == "Links"{
            by=ay; bx = ax-dx /2;
            cy=dy; cx=bx;
        }else if richtung == "Rechts"{
            by=ay; bx = dx-ax /2;
            cy=dy; cx=bx;
        }
        self.koord[2]=(bx,by);
        self.koord[3]=(cx,cy);
    }
}

pub fn sammle_relationen(content: &String) -> Vec<String> {
    let re = Regex::new(r"V\{(?P<text>[^\}]+)\}")
        .unwrap();
    parser::parse_text_tovector(&content, &re)
}

pub fn sammle_startknoten(content: &String) -> String {
    let re = Regex::new(r".*\((?P<text>[\w]+)/.*\)")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

pub fn sammle_endknoten(content: &String) -> String {
    let re = Regex::new(r".*\(.*/(?P<text>[\w]+)\)")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

pub fn sammle_typ(content: &String) -> String {
    let re = Regex::new(r".*\(.*/.*\).*typ:(?P<text>.?[\w^;]+.?)")
        .unwrap();
    let mut value: String;
    let temp: String = parser::parse_text_to_string(&content, &re);
    let re = Regex::new(r"(?P<text>[\w]+)")
        .unwrap();
    let temp2: String = parser::parse_text_to_string(&temp, &re);
    let s = temp2.as_str();
    match s {
        "komp" => value = "Komposition".to_string(),//schwarze raute
        "aggr" => value = "Aggregation".to_string(), //weiße raute
        "impl" => value = "Implementierung".to_string(), // gestrichelt weißer ausgefüllter Pfeil
        "inher" => value = "Vererbung".to_string(),// durchgezogen weißer ausgefüllter Pfeil
        "abh" => value = "Abhängigkeit".to_string(),// gestrichelter pfeil --->
        "einfach" => value = "Einfache Abhängigkeit".to_string(), //durchgezogener Pfeil
        _ => value = "Einfache Abhängigkeit".to_string()
    }
    value
}

pub fn sammle_m1(content: &String) -> String {
    let re = Regex::new(r".*\(.*/.*\).*typ:.*m1:(?P<text>[^;]+)")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

pub fn sammle_m2(content: &String) -> String {
    let re = Regex::new(r".*\(.*/.*\).*typ:.*m2:(?P<text>[^;]+)")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

pub fn sammle_beschr(content: &String) -> String {
    let re = Regex::new(r".*\(.*/.*\).*typ:.*desc:(?P<text>[^;&&\s]+)")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

pub fn baue_relationen(input: &String, klassen: &mut Vec<klassendiagramm::Klasse>) -> Vec<Relation> {
    let v: Vec<String> = sammle_relationen(input).clone();
    let mut relationen: Vec<Relation> = vec![];
    for relation in v.iter() {
        let s = sammle_startknoten(&relation);
        let e = sammle_endknoten(&relation);
        let t = sammle_typ(&relation);
        let mut temp: Vec<String> = vec![];
        temp.push(t);
        let m1 = sammle_m1(&relation);
        temp.push(m1);
        let m2 = sammle_m2(&relation);
        temp.push(m2);
        let beschr = sammle_beschr(&relation);
        temp.push(beschr);
        if s.chars().count() > 0 && e.chars().count() > 0 {
            //pruefe ob Relation gültig ist.
            let mut start_klasse = Klasse::default();
            let mut end_klasse = Klasse::default();
            ;
            let mut start: bool = false;
            let mut end: bool = false;
            for klasse in klassen.iter_mut() {
                if klasse.get_id().first().unwrap() == &s {
                    klasse.add_ausgehend();
                    start_klasse = klasse.clone();
                    start = true;
                }
                if klasse.get_id().first().unwrap() == &e {
                    klasse.add_eingehend();
                    end_klasse = klasse.clone();
                    end = true;
                }
            }
            if start && end {
                relationen.push(build_relation(s.clone(), e.clone(), temp
                                               , start_klasse, end_klasse));
            }
        }
    }
    relationen
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bestimme_richtung_oben() {
        assert_eq!("Oben", Relation::bestimme_richtung(1,1,1,0));
        //diagpnale test
        assert_eq!("Oben", Relation::bestimme_richtung(1,1,0,0));
        //negative value test
        assert_eq!("Oben", Relation::bestimme_richtung(-1,-1,-1,-2));
    }
    #[test]
    fn bestimme_richtung_links() {
        assert_eq!("Links", Relation::bestimme_richtung(1,1,0,1));
        //Linke grenze
        assert_eq!("Links", Relation::bestimme_richtung(1,1,0,2));
    }
    #[test]
    fn bestimme_richtung_rechts() {
        assert_eq!("Rechts", Relation::bestimme_richtung(1,1,2,1));
        assert_eq!("Rechts", Relation::bestimme_richtung(1,1,2,0));
    }
    #[test]
    fn bestimme_richtung_unten() {
        assert_eq!("Unten", Relation::bestimme_richtung(1,1,1,2));
        assert_eq!("Unten", Relation::bestimme_richtung(1,1,2,2));
    }
}

