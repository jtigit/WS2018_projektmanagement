extern crate regex;

use self::regex::Regex;
use crate::parser::parser;
use crate::parser::relation::Relation;
use crate::parser::relation;
use crate::layout::graphbuilder;
use crate::image_gen::image_gen;
use image::{Rgb, RgbImage, ImageBuffer};

#[derive(Debug, Default,Clone)]
pub struct Klasse {
    x: f32,
    y: f32,
    id: Vec<String>,
    attribute: Vec<String>,
    methoden: Vec<String>,
    pk:usize ,
    //Variable für das Positionieren
    anzahl_ausgehender :u32 ,
    anzahl_eigehender :u32
}
impl Klasse {
    pub fn get_laenge_breite(&self) -> (u32, u32) {
        let mut l: u32 = 0;
        let mut b: u32 = 0;
        let _temp: u32 = 0;

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
    pub fn get_x(&self) -> &f32 { &self.x }
    pub fn get_y(&self) -> &f32 { &self.y }
    pub fn get_id(&self) -> &Vec<String> {
        &self.id
    }
    pub fn _get_atr(&self) -> &Vec<String> {
        &self.attribute
    }
    pub fn _get_meth(&self) -> &Vec<String> {
        &self.methoden
    }
    pub fn add_eingehend(&mut self){
        self.anzahl_eigehender=self.anzahl_eigehender+1;
    }
    pub fn add_ausgehend(&mut self){
        self.anzahl_ausgehender=self.anzahl_ausgehender+1;
    }
    pub fn _get_eingehend(&self) -> &u32 {
        &self.anzahl_eigehender
    }
    pub fn _get_ausgehend(&self) -> &u32 {
        &self.anzahl_ausgehender
    }
    pub fn get_pk(&self)-> &usize {&self.pk}
    pub fn get_pos_x(&self)-> &f32 {&self.x}
    pub fn get_pos_y(&self)-> &f32 {&self.y}
    pub fn set_pk(&mut self,value:usize){
        self.pk=value;
    }
    pub fn set_pos_x(&mut self,value:f32){
        let bildgroesse:f32=2500.0;
        let rand =bildgroesse*0.1;
        self.x=value*(bildgroesse-3.0*rand)+rand*0.1;
    }
    pub fn set_pos_y(&mut self,value:f32){
        let bildgroesse:f32=2500.0;
        let rand =bildgroesse*0.1;
        self.y=value*(bildgroesse-4.0*rand)+rand;
    }
}
pub struct Klassendiagramm {
    klassen: Vec<Klasse>,
    relationen: Vec<Relation>
}
impl Klassendiagramm{
    pub fn get_klassen(&self)->&Vec<Klasse>{
        &self.klassen
    }
    pub fn _get_relationen(&self)->&Vec<Relation>{
        &self.relationen
    }

    pub fn get_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        image_gen::draw_klassendiagramm(&self)
    }
}

//Aus dem Text werden Klassendiagramme gebaut.
pub fn parse_klassendiagramme(input: &String)->Vec<Klassendiagramm> {
    // Splitte den Text in Teilstücke
    let v2 : Vec<&str>= input.split("Klassendiagramm").collect();
    let mut v: Vec<String> = vec![];
    //Konvertierung in String mit ausnahmebehandlung
    for s in v2{
        if s != ""{
            v.push(s.to_string());
        }
    }
    let mut klassendiagramme: Vec<Klassendiagramm> = vec![];
    for klassendiagramm in v.iter() {
        let mut klassen = baue_klassen(klassendiagramm);
        let mut relationen = relation::baue_relationen(klassendiagramm, &mut klassen);
        if !klassen.is_empty() {
            graphbuilder::create_layout(&mut klassen,&mut relationen);
            //graphbuilder::create_graph(&mut klassen,&mut relationen);
            let a:Klassendiagramm = Klassendiagramm{klassen:klassen.clone(),relationen:relationen.clone()};
            klassendiagramme.push(a);
        }
    }

    klassendiagramme
}

//Standard Konstruktor
fn build_klasse(id: Vec<String>, attribute: Vec<String>, methoden: Vec<String>) -> Klasse {
    let pk;

    let anzahl_eigehender=0;
    let anzahl_ausgehender=0;
    unsafe {
        parser::count_all_objects();
        pk = parser::get_counter();
    }
    Klasse { x: 0.0, y: 0.0, id, attribute, methoden,pk ,anzahl_ausgehender,anzahl_eigehender}
}

 fn sammle_klassen(content: &String) -> Vec<String> {
    let re = Regex::new(r"Klasse\{(?P<text>[^\}]+)}")
        .unwrap();
    parser::parse_text_tovector(&content, &re)
}
 fn _sammle_klassendiagramme(content: &String) -> Vec<String> {
    let re = Regex::new(r"(?s)Klassendiagramm\{(?P<text>.*)[}Klassendiagramm]{1}")
        .unwrap();
    parser::parse_text_tovector(&content, &re)
}

 fn sammle_klassen_namen(content: &String) -> String {
    let re = Regex::new(r"name:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

 fn sammle_klassen_typ(content: &String) -> String {
    let re = Regex::new(r"typ:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
     let mut value: String;
     let temp:String =  parser::parse_text_to_string(&content, &re);
     let re = Regex::new(r"(?P<text>[\w]+)")
         .unwrap();
     let temp2:String =  parser::parse_text_to_string(&temp, &re);
     let s = temp2.as_str();
     match s {
         "interface" => value =  "<<interface>>".to_string(),
         "JavaBean" => value = ">>javaBean>>".to_string(),
         "Message" => value = "<<Message>>".to_string(),
         "abstract" => value = "<<abstract>>".to_string(),
         _ => value = "".to_string()
     }
     value
}

 fn sammle_klassen_atr(content: &String) -> String {
    let re = Regex::new(r"(?s)[-]{2,}[\r\n]{1,}(?P<text>.*)[\r\n]{1,}[-]{2,}")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

fn sammle_klassen_meth(content: &String) -> String {
    let re = Regex::new(r"(?s)[-]{2,}[\r\n]{1,}.*[-]{2,}[\r\n]{1,}(?P<text>.*)[\r\n]*")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

 fn sammle_argumente(content: &String) -> Vec<String> {
    let re = Regex::new(r"(?s)(?P<text>[^\r\n]+)")
        .unwrap();
    let v: Vec<String> = parser::parse_text_tovector(&content, &re);
    v
}

 fn baue_klassen(input: &String)->Vec<Klasse>{
    let _name = String::from("");
    let _typ = String::from("");
    let _atr: Vec<String> = vec![];
    let _meth: Vec<String> = vec![];
    let v: Vec<String> = sammle_klassen(input).clone();
    let mut klassen: Vec<Klasse> = vec![];
    for klasse in v.iter() {
        let n = sammle_klassen_namen(klasse);
        println!("Klasse: {}",&n);
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

            let (_t1, _t2) = (k.get_laenge_breite().0, k.get_laenge_breite().1);
        }

    }
    klassen
}