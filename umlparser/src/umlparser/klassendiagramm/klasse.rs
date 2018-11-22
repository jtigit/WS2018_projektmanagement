extern crate regex;

use std::fs;
use regex::Regex;
use crate::umlparser::parser;

#[derive(Clone)]
pub struct Klasse {
    x: u32,
    y: u32,
    id: Vec<String>,
    attribute: Vec<String>,
    methoden: Vec<String>,
    pk:u32 ,
    //Variable für das Positionieren
    anzahl_ausgehender :u32 ,
    anzahl_eigehender :u32
}

impl Klasse {
    pub fn get_laenge_breite(&self) -> (u32, u32) {
        let mut l: u32 = 0;
        let mut b: u32 = 0;
        let mut temp: u32 = 0;

        for arg in &self.id {
            let temp = arg.chars().count() as u32;
            if temp > b { b = temp; }
            if temp > 0 { l += 1; }
        }
        for arg in &self.attribute {
            let temp = arg.chars().count() as u32;
            if temp > b { b = temp; }
            if temp > 0 { l += 1; }
        }
        for arg in &self.methoden {
            let temp = arg.chars().count() as u32;
            if temp > b { b = temp; }
            if temp > 0 { l += 1; }
        }
        (l, b)
    }
    pub fn get_id(&self) -> &Vec<String> {
        &self.id
    }
    pub fn get_atr(&self) -> &Vec<String> {
        &self.attribute
    }
    pub fn get_meth(&self) -> &Vec<String> {
        &self.methoden
    }
    pub fn add_eingehend(&mut self){
        self.anzahl_eigehender=self.anzahl_eigehender+1;
    }
    pub fn add_ausgehend(&mut self){
        self.anzahl_ausgehender=self.anzahl_ausgehender+1;
    }
    pub fn get_eingehend(&self) -> &u32 {
        &self.anzahl_eigehender
    }
    pub fn get_ausgehend(&self) -> &u32 {
        &self.anzahl_ausgehender
    }
}

struct Klassendiagramm {
    klassen: Vec<Klasse>
}

//Standard Konstruktor
fn build_klasse(id: Vec<String>, attribute: Vec<String>, methoden: Vec<String>) -> Klasse {
    let pk;

    let mut anzahl_eigehender=0;
    let mut anzahl_ausgehender=0;
    unsafe {
        parser::count_all_objects();
        pk = parser::get_counter();
    }
    Klasse { x: 0, y: 0, id, attribute, methoden,pk ,anzahl_ausgehender,anzahl_eigehender}
}

pub fn sammle_klassen(content: &String) -> Vec<String> {
    let re = Regex::new(r"Klasse\{(?P<text>[^\}]+)\}")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    parser::parse_text(&content, &re)
}

pub fn sammle_klassen_namen(content: &String) -> String {
    let re = Regex::new(r"name:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    let mut v: Vec<String> = parser::parse_text(&content, &re);
    if v.len() == 0 {
        return "".to_string();
    } else {
        let k: &str = v.get(0).unwrap();
        return k.to_string();
    }
}

pub fn sammle_klassen_typ(content: &String) -> String {
    let re = Regex::new(r"typ:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    let mut v: Vec<String> = parser::parse_text(&content, &re);
    if v.len() == 0 {
        return "".to_string();
    } else {
        let k: &str = v.get(0).unwrap();
        return k.to_string();
    }
}

pub fn sammle_klassen_atr(content: &String) -> String {
    let re = Regex::new(r"(?s)[-]{2,}[\r\n]{1,}(?P<text>.*)[\r\n]{1,}[-]{2,}")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    let mut v: Vec<String> = parser::parse_text(&content, &re);
    if v.len() == 0 {
        return "".to_string();
    } else {
        let k: &str = v.get(0).unwrap();
        return k.to_string();
    }
}

pub fn sammle_klassen_meth(content: &String) -> String {
    let re = Regex::new(r"(?s)[-]{2,}[\r\n]{1,}.*[-]{2,}[\r\n]{1,}(?P<text>.*)[\r\n]*")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    let mut v: Vec<String> = parser::parse_text(&content, &re);
    if v.len() == 0 {
        return "".to_string();
    } else {
        let k: &str = v.get(0).unwrap();
        return k.to_string();
    }
}

pub fn sammle_argumente(content: &String) -> Vec<String> {
    let re = Regex::new(r"(?s)(?P<text>[^\r\n]+)")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    let mut v: Vec<String> = parser::parse_text(&content, &re);
    v
}

pub fn baue_klassen(input: &String)->Vec<Klasse>{
    let name = String::from("");
    let typ = String::from("");
    let atr: Vec<String> = vec![];
    let meth: Vec<String> = vec![];
    let v: Vec<String> = sammle_klassen(input).clone();
    let mut klassen: Vec<Klasse> = vec![];
    for klasse in v.iter() {
        let n = sammle_klassen_namen(klasse);
        let t = sammle_klassen_typ(klasse);
        let atribute = sammle_klassen_atr(klasse);
        // Trenne Attribute von next line und carriege return
        let a: Vec<String> = sammle_argumente(&atribute);
        let methode = sammle_klassen_meth(klasse);
        let m: Vec<String> = sammle_argumente(&methode);
        if n.chars().count()>0 {
            let i: Vec<String> = vec! {n, t};
            let k: Klasse = build_klasse(i, a, m);
            let clonek = k.clone();
            klassen.push(clonek);
            let (t1, t2) = (k.get_laenge_breite().0, k.get_laenge_breite().1);
            println!("Klasse \" {} \" hat Länge: {} und Breite: {}", k.get_id().get(0).unwrap(), t1, t2);
        }
    }
    klassen
}


