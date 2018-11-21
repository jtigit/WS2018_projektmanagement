extern crate regex;

use std::fs;
use regex::Regex;

#[derive(Clone)]
pub struct Klasse {
    x :u32 ,y:u32,
    id: Vec<String>,
    attribute: Vec<String>,
    methoden: Vec<String>,
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
    pub fn get_id(&self)->&Vec<String>{
        &self.id
    }
    pub fn get_atr(&self)->&Vec<String>{
        &self.attribute
    }
    pub fn get_meth(&self)->&Vec<String>{
        &self.methoden
    }
}

struct Klassendiagramm {
    klassen: Vec<Klasse>
}
//Standard Konstruktor
fn build_klasse(id: Vec<String>, attribute: Vec<String>, methoden: Vec<String>)->Klasse{
    Klasse{x:0,y:0,id,attribute,methoden}
}

pub fn sammle_klassen(content: &String) -> Vec<String> {
    let re = Regex::new(r"Klasse\{(?P<text>[^\}]+)\}")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    parse_text(&content, &re)
}

pub fn sammle_klassen_namen(content: &String) -> String {
    let re = Regex::new(r"name:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    let mut v: Vec<String> = parse_text(&content, &re);
    if v.len() == 0 {
        return "".to_string();
    } else {
        let k: &str = v.get(0).unwrap();
        return k.to_string();
    }
}

pub fn parse_text(text: &String, re: &Regex) -> Vec<String> {
    let mut v: Vec<String> = vec![];

    for caps in re.captures_iter(&text) {
        let text = caps.get(1).unwrap().as_str();
        v.push(text.to_string());
        println!("Element: {:?}", &caps["text"]);
    }
    v
}

pub fn sammle_klassen_typ(content: &String) -> String {
    let re = Regex::new(r"typ:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    //Übergebe dem Parser den Text und den Regulären Ausdruck
    let mut v: Vec<String> = parse_text(&content, &re);
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
    let mut v: Vec<String> = parse_text(&content, &re);
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
    let mut v: Vec<String> = parse_text(&content, &re);
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
    let mut v: Vec<String> = parse_text(&content, &re);
    v
}

pub fn baue_klassen() {
    let name = String::from("");
    let typ = String::from("");
    let atr: Vec<String> = vec![];
    let meth: Vec<String> = vec![];
    let v: Vec<String> = sammle_klassen(&readFile()).clone();
    let mut klassen: Vec<Klasse> = vec![];
    for klasse in v.iter() {
        let n = sammle_klassen_namen(klasse);
        let t = sammle_klassen_typ(klasse);
        let atribute = sammle_klassen_atr(klasse);
        // Trenne Attribute von next line und carriege return
        let a: Vec<String> = sammle_argumente(&atribute);
        let methode = sammle_klassen_meth(klasse);
        let m: Vec<String> = sammle_argumente(&methode);
        let i: Vec<String> = vec!{n,t};
        let k: Klasse = build_klasse(i,a,m);
        let clonek = k.clone();
        klassen.push(clonek);
        let (t1, t2) = (k.get_laenge_breite().0, k.get_laenge_breite().1);
        println!("Klasse \" {} \" hat Länge: {} und Breite: {}", k.get_id().get(0).unwrap(), t1, t2);
    }
}

pub fn readFile() -> String {
    let filename = String::from("example.txt");
    let contents = fs::read_to_string(&filename);
    let text = contents.unwrap();
    text
}