extern crate tera;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

use std::io;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use tera::Context;

#[derive(FromForm)]
struct Eingabe {
    value: String,
    action: String
}


#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}

#[get("/")]
pub fn index(flash: Option<FlashMessage>) -> Template {
    let mut context = Context::new();

    let s = flash.map(|msg| format!("{}", msg.msg())).unwrap_or_else(|| "abc -> def".to_string());

    context.add("input_message", &s);
    Template::render("content", &context)
}

#[post("/submit", data = "<eingabe>")]
fn submit_task(eingabe: Form<Eingabe>) -> Flash<Redirect> {
    let x: Eingabe = eingabe.into_inner();

    if("send".eq(&x.action)) {
        Flash::success(Redirect::to("/"), &x.value)
    } else if("save".eq(&x.action)) {
        Flash::error(Redirect::to("/"), &x.value)
    } else if("delete".eq(&x.action)) {
        Flash::success(Redirect::to("/"), "")
    } else {
        Flash::error(Redirect::to("/"), &x.value)
    }
}