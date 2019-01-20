extern crate regex;

use crate::parser::useCaseDiagramm::usecasediagramm::Usecasediagramm;
use crate::parser::useCaseDiagramm::usecase::Usecase;
use crate::parser::useCaseDiagramm::actor::Actor;
use crate::parser::useCaseDiagramm::boundary::Boundary;
use crate::parser::useCaseDiagramm::relation::Relation;
use crate::parser::parser;
use self::regex::Regex;


pub fn build_usecasediagramm(input: &String) -> Vec<Usecasediagramm> {
    // Splitte den Text in Teilstücke
    let v2: Vec<&str> = input.split("Usecasediagramm").collect();
    let mut v: Vec<String> = vec![];
    //Konvertierung in String mit ausnahmebehandlung
    for s in v2 {
        if s != "" {
            v.push(s.to_string());
        }
    }
    let mut usecasediagramme: Vec<Usecasediagramm> = vec![];
    for Usecasediagramm in v.iter() {
        let mut usecases = baue_usecases(Usecasediagramm);
        let mut actors = baue_actors(Usecasediagramm);
        let mut boundarys = baue_boundarys(Usecasediagramm);
        let mut relations = baue_relations(Usecasediagramm);
        let result: Usecasediagramm = Usecasediagramm { usecases, actors, boundarys, relations };
        usecasediagramme.push(result);
    }
    usecasediagramme
}
//Erste Ebene
fn baue_usecases(input: &String)->Vec<Usecase>{
    let mut bezeichung = String::from("");
    let mut extension_text = String::from("");
    let v: Vec<String> = sammle_usecases(input).clone();
    let mut usecases: Vec<Usecase> = vec![];

    for usecase in v.iter() {
        bezeichung = sammle_usecase_bezeichnung(usecase);
        extension_text= sammle_usecase_extension_text(usecase);
        if extension_text.chars().count()>0{
            let u :Usecase = Usecase::new_extension(bezeichung,extension_text);
            usecases.push(u);
        }else{
            let u :Usecase = Usecase::new(bezeichung);
            usecases.push(u);
        }
    }
    usecases
}
fn baue_actors(input: &String)->Vec<Actor>{
    let mut bezeichung = String::from("");
    let v: Vec<String> = sammle_actors(input).clone();
    let mut actors: Vec<Actor> = vec![];


    for actor in v.iter() {
        bezeichung = sammle_actor_bezeichnung(actor);
        if bezeichung.chars().count()>0{
            let a :Actor = Actor::new(bezeichung);
            actors.push(a);
        }else{
        }
    }
    actors
}
fn baue_boundarys(input: &String)->Vec<Boundary>{
    let mut bezeichung = String::from("");
    let v: Vec<String> = sammle_boundarys(input).clone();
    let mut boundarys: Vec<Boundary> = vec![];


    for boundary in v.iter() {
        bezeichung = sammle_boundary_bezeichnung(boundary);
        if bezeichung.chars().count()>0{
            let b :Boundary = Boundary::new(bezeichung);
            boundarys.push(b);
        }else{
        }
    }
    boundarys
}
fn baue_relations(input: &String)->Vec<Relation>{
    let mut bezeichung = String::from("");
    let mut start = String::from("");
    let mut ende = String::from("");
    let v: Vec<String> = sammle_relations(input).clone();
    let mut relations: Vec<Relation> = vec![];


    for relation in v.iter() {
        bezeichung = sammle_relation_typ(relation);
        start = sammle_relation_startknoten(relation);
        ende = sammle_relation_endknoten(relation);

        let mut r:Relation=Relation::new();
        if bezeichung == "Inklusion"{
            r = Relation::new_include();


        }else if bezeichung == "Extension"{
            r = Relation::new_extend();
        }else{

        }
        relations.push(r);
    }
    relations
}
//Zweite Ebene-------------------
//Usecase
fn sammle_usecases(content: &String) -> Vec<String> {
    let re = Regex::new(r"Usecase\{(?P<text>[^\}]+)}")
        .unwrap();
    parser::parse_text_tovector(&content, &re)
}
fn sammle_usecase_bezeichnung(content: &String) -> String {
    let re = Regex::new(r"name:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}
fn sammle_usecase_extension_text(content: &String) -> String {
    let re = Regex::new(r"extension:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}
//Actors
fn sammle_actors(content: &String) -> Vec<String> {
    let re = Regex::new(r"Actor\{(?P<text>[^\}]+)}")
        .unwrap();
    parser::parse_text_tovector(&content, &re)
}
fn sammle_actor_bezeichnung(content: &String) -> String {
    let re = Regex::new(r"name:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}
//Boundarys
fn sammle_boundarys(content: &String) -> Vec<String> {
    let re = Regex::new(r"Boundary\{(?P<text>[^\}]+)}")
        .unwrap();
    parser::parse_text_tovector(&content, &re)
}
fn sammle_boundary_bezeichnung(content: &String) -> String {
    let re = Regex::new(r"name:[\W]*(?P<text>[\w]+)[^\w]")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}
//Relation
pub fn sammle_relations(content: &String) -> Vec<String> {
    let re = Regex::new(r"V\{(?P<text>[^\}]+)\}")
        .unwrap();
    parser::parse_text_tovector(&content, &re)
}

pub fn sammle_relation_startknoten(content: &String) -> String {
    let re = Regex::new(r".*\((?P<text>[\w]+)/.*\)")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

pub fn sammle_relation_endknoten(content: &String) -> String {
    let re = Regex::new(r".*\(.*/(?P<text>[\w]+)\)")
        .unwrap();
    parser::parse_text_to_string(&content, &re)
}

pub fn sammle_relation_typ(content: &String) -> String {
    let re = Regex::new(r".*\(.*/.*\).*typ:(?P<text>.?[\w^;]+.?)")
        .unwrap();
    let mut value: String;
    let temp: String = parser::parse_text_to_string(&content, &re);
    let re = Regex::new(r"(?P<text>[\w]+)")
        .unwrap();
    let temp2: String = parser::parse_text_to_string(&temp, &re);
    let s = temp2.as_str();
    match s {
        "include" => value = "Inklusion".to_string(),// gestrichelter pfeil --->
        "extend" => value = "Extension".to_string(), // gestrichelter pfeil --->
        _ => value = "Einfache Abhängigkeit".to_string()//durchgezogene Linie
    }
    value
}
