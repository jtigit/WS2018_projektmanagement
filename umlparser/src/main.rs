#![feature(proc_macro_hygiene, decl_macro, plugin, non_ascii_idents)]
#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate tera;
extern crate rand;
extern crate image;
extern crate imageproc;
extern crate rusttype;

mod parser;
mod routes;
mod image_gen;
mod layout;

use std::fs;
use rocket_contrib::templates::Template;
use crate::parser::klassendiagramm::klassendiagramm::Klassendiagramm;
use crate::parser::useCaseDiagramm::usecasediagramm::Usecasediagramm;

fn main() {

    start_rocket();
    //test();
    //test_usecase();
    //test_actor();
    //test_boundary();
    //test_relation();
}

fn test() {
    let kdv: Vec<Klassendiagramm> = parser::parser::starte_umlparser(&read_file("example.txt".to_string())).get_klassendiagramme();
    let kd : &Klassendiagramm = kdv.get(0).unwrap();
    let k = kd.get_klassen();
    let r = kd._get_relationen();
    for klasse in k {
        println!("Main:::Klassenname:{:?}  Koordinaten({}/{})",klasse.get_id().first(),klasse.get_pos_x(),klasse.get_pos_y());
    }
    for relation in r {
        println!("Main:::Relation: typ:{},  m1:{}   , m2:{}",relation.get_typ()
        ,relation.get_beschr_startknoten(),relation.get_beschr_endknoten());
    }
}
fn test_usecase() {
    let kdv: Vec<Usecasediagramm> = parser::parser::starte_umlparser(&read_file("usecase_example.txt".to_string()))
        .get_usecasediagramme();
    let kd:&Usecasediagramm = kdv.get(0).unwrap();
    let u = &kd.usecases;
    for usecase in u {
        if usecase.extension {
            println!("Main::Usecase  : {} +  {} ", usecase.text.name, usecase.extension_text.name);
        }else{
            println!("Main::Usecase  : {}  ",usecase.text.name);
        }
    }
}
fn test_actor() {
    let kdv: Vec<Usecasediagramm> = parser::parser::starte_umlparser(&read_file("usecase_example.txt".to_string()))
        .get_usecasediagramme();
    let kd:&Usecasediagramm = kdv.get(0).unwrap();
    let u = &kd.actors;
    for actor in u {
            println!("Main::Actor  : {}  ",actor.art.name);
    }
}
fn test_boundary() {
    let kdv: Vec<Usecasediagramm> = parser::parser::starte_umlparser(&read_file("usecase_example.txt".to_string()))
        .get_usecasediagramme();
    let kd:&Usecasediagramm = kdv.get(0).unwrap();
    let u = &kd.boundarys;
    for boundary in u {
        println!("Main::Boundary  : {}  ",boundary.text.name);
    }
}
fn test_relation() {
    let kdv: Vec<Usecasediagramm> = parser::parser::starte_umlparser(&read_file("usecase_example.txt".to_string()))
        .get_usecasediagramme();
    let kd:&Usecasediagramm = kdv.get(0).unwrap();
    let u = &kd.relations;
    for relation in u {
        println!("Main::relation  : start:{} , ende:{} , typ: {} "
                 ,relation.start.name,relation.ende.name,relation.text.name);
    }
}



fn start_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                 routes::routes::index,
                 routes::routes::about,
                 routes::routes::howto,
                 routes::routes::static_content,
                 routes::routes::img
            ]
        ).mount(
        "/api",
        routes![
                 routes::routes::submit_task,

            ]
        ).attach(Template::fairing()) // Für Templates
        .launch();
}
///Liest konkret example.txt ein und gibt den Inhalt des Textes als String zuruck
pub fn read_file(file:String) -> String {
    let filename = String::from(file);
    let contents = fs::read_to_string(&filename);
    let text = contents.unwrap();
    text
}