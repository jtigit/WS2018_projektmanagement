#![feature(proc_macro_hygiene, decl_macro, plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

mod routes;

use std::io;
use rocket_contrib::Template;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;



#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}

fn main() {
    start_rocket();
}

fn start_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                 routes::apiroute::index,
                 static_content
            ]
        ).mount(
        "/api",
        routes![
                 routes::apiroute::submit_task
            ]
        ).attach(Template::fairing()) // Für Templates
        .launch();
}