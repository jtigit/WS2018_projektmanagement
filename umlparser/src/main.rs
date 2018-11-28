#![feature(proc_macro_hygiene, decl_macro, plugin, custom_derive, non_ascii_idents)]
#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate tera;

extern crate image;
extern crate imageproc;
extern crate rusttype;

mod parser;
mod routes;
mod image_gen;

use std::fs;
use rocket_contrib::templates::Template;

fn main() {
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

pub fn read_file() -> String {
    let filename = String::from("example.txt");
    let contents = fs::read_to_string(&filename);
    let text = contents.unwrap();
    text
}