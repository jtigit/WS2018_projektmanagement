extern crate regex;

use self::regex::Regex;
use crate::parser::parser;
use crate::parser::klassendiagramm::klassendiagramm;

#[derive(Clone)]
pub struct Relation {
    //Können mehrere Punkte sein. erster und letzer sind start und end
    koord: Vec<(u32,u32)>,
    verbindet: (String,String),
    // Typ , m1,m2
    beschreibung: Vec<String>,
    pk:usize
}
//Standard Konstruktor
pub fn build_relation(startknoten: String, endknoten:String, beschreibung:Vec<String>) -> Relation {
    let a:(u32,u32)=(0,0);
    let mut koord:Vec<(u32,u32)> = vec![];
    koord.push(a);koord.push(a);
    let pk;// primary key
    unsafe {
        parser::count_all_objects();
        pk = parser::get_counter();
    }
    Relation { koord, verbindet : (startknoten, endknoten),beschreibung,pk}
}
impl Relation {
    pub fn get_x_startknoten(&self) -> &u32 {
        let (x,y) = self.koord.first().unwrap();
        x
    }
    pub fn get_y_startknoten(&self) -> &u32 {
        let (x,y) = self.koord.first().unwrap();
        y
    }
    pub fn get_x_endknoten(&self) -> &u32 {
        let (x,y) = self.koord.last().unwrap();
        x
    }
    pub fn get_y_endknoten(&self) -> &u32 {
        let (x,y) = self.koord.last().unwrap();
        y
    }
    pub fn get_name_startknoten(&self) -> &String {
        let(x,y) = &self.verbindet;&x
    }
    pub fn get_name_endknoten(&self) -> &String {
        let(x,y) = &self.verbindet;&y
    }
    pub fn get_typ(&self) -> &String {
        &self.beschreibung.first().unwrap()
    }
    pub fn get_beschr_startknoten(&self) -> &String {
        &self.beschreibung.get(1).unwrap()
    }
    pub fn get_beschr_endknoten(&self) -> &String {
        &self.beschreibung.get(1).unwrap()
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

    parser::parse_text_to_string(&content, &re)

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

pub fn baue_relationen(input: &String, klassen: &mut Vec<klassendiagramm::Klasse>) ->Vec<Relation>{
    let name = String::from("");
    let typ = String::from("");
    let atr: Vec<String> = vec![];
    let meth: Vec<String> = vec![];
    let v: Vec<String> = sammle_relationen(input).clone();
    let mut relationen: Vec<Relation> = vec![];
    for relation in v.iter() {
        let s = sammle_startknoten(&relation);
        let e = sammle_endknoten(&relation);
        let t = sammle_typ(&relation);
        let mut temp: Vec<String> = vec![];
        let m1 = sammle_m1(&relation);temp.push(m1);
        let m2 = sammle_m2(&relation);temp.push(m2);
        let beschr = sammle_beschr(&relation);temp.push(beschr);
        if s.chars().count()>0 && e.chars().count()>0{
            //pruefe ob Relation gültig ist.
            let mut start:bool = false;
            let mut end:bool = false;
            for klasse in klassen.iter_mut(){
                if klasse.get_id().first().unwrap()==&s {
                    klasse.add_ausgehend();
                    start=true;
                }
                if klasse.get_id().first().unwrap()==&e {
                    klasse.add_eingehend();
                    end=true;
                }
            }
            if start && end {
                relationen.push( build_relation(s.clone(), e.clone(), temp));
            }
        }
}
    relationen
}

