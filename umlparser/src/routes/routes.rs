extern crate tera;


use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

use std::io;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use tera::Context;

#[derive(FromForm)]
pub struct Eingabe {
    value: String,
    action: String
}


#[get("/static/<file..>")]
pub fn static_content(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}

#[get("/")]
pub fn index(flash: Option<FlashMessage>) -> Template {
    let mut context = Context::new();

    let s = flash.map(|msg| format!("{}", msg.msg())).unwrap_or_else(|| "abc -> def".to_string());

    context.add("input_message", &s);
    context.add("replaceImage", &"https://via.placeholder.com/800x800");
    Template::render("content", &context)
}

#[get("/about")]
pub fn about() -> Template {
    let context = Context::new();
    Template::render("about", &context)
}

#[get("/howto")]
pub fn howto() -> Template {
    let context = Context::new();
    Template::render("howto", &context)
}

#[post("/submit", data = "<eingabe>")]
pub fn submit_task(eingabe: Form<Eingabe>) -> Flash<Redirect> {
    let x: Eingabe = eingabe.into_inner();

    if "send".eq(&x.action) {
        Flash::success(Redirect::to("/"), &x.value)
    } else if "delete".eq(&x.action) {
        Flash::success(Redirect::to("/"), "")
    } else {
        Flash::error(Redirect::to("/"), &x.value)
    }
}