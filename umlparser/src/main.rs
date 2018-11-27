#![feature(proc_macro_hygiene, decl_macro, plugin, custom_derive, non_ascii_idents)]
#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate tera;
extern crate rand;

mod parser;
mod routes;
mod layout;

use std::fs;
use rocket_contrib::templates::Template;
use crate::parser::klassendiagramm::klassendiagramm::Klassendiagramm;

fn main() {
    let kdv: Vec<Klassendiagramm> = parser::parser::starte_umlparser(&read_file()).get_klassendiagramme();
    let kd : &Klassendiagramm = kdv.get(0).unwrap();
    let k = kd.get_klassen();
    let r = kd._get_relationen();
    for klasse in k {
        println!("Main:::Klassenname:{:?}  Koordinaten({}/{})",klasse.get_id().first(),klasse.get_pos_x(),klasse.get_pos_y());
    }
    for relation in r {
        println!("Main:::Relation:  typ:{}",relation.get_typ());
    }
    start_rocket();
}

fn start_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                 routes::routes::index,
                 routes::routes::about,
                 routes::routes::howto,
                 routes::routes::static_content
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
pub fn read_file() -> String {
    let filename = String::from("example.txt");
    let contents = fs::read_to_string(&filename);
    let text = contents.unwrap();
    text
}