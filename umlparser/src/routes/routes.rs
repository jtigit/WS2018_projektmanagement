extern crate tera;
extern crate time;

use std::path::{Path, PathBuf};
use std::option::Option;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use rocket::response::NamedFile;
use rocket::request::FromParam;

use std::io;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use tera::Context;
use rocket::http::RawStr;

use crate::parser::parser;
use crate::parser::klassendiagramm::klassendiagramm::Klassendiagramm;
use crate::parser::useCaseDiagramm::usecasediagramm;

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

    let kdv: Vec<Klassendiagramm> = parser::starte_umlparser(&s).get_klassendiagramme();
    //let usv: Vec<Usecasediagramm> = parser::starte_umlparser(&s).get_usecasediagramme();

    if kdv.get(0).is_some() {
        let kd : &Klassendiagramm = kdv.get(0).unwrap();
        let k = kd.get_klassen();
        let r = kd._get_relationen();
        for klasse in k {
            println!("Main:::Klassenname:{:?}  Koordinaten({}/{})",klasse.get_id().first(),klasse.get_pos_x(),klasse.get_pos_y());
        }
        for relation in r {
            println!("Main:::Relation:  typ:{}",relation.get_typ());
        }

        let now = time::get_time();

        kd.get_image().save(format!("static/temp/{:?}.png", now.nsec)).unwrap();

        context.add("replaceImage", &(format!("http://localhost:8000/static/temp/{:?}.png", now.nsec)));
    } /*else if usv.get(0).is_some() {
        let uc : &Usecasediagramm = usv.get(0).unwrap();
        let now = time::get_time();

        println!("DEBUG");

    } */else {
        context.add("replaceImage", &"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNgYAAAAAMAASsJTYQAAAAASUVORK5CYII=");
    }

    context.add("input_message", &s);
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

    println!("Test");
    if "send".eq(&x.action) {
        println!("Test");
        Flash::success(Redirect::to("/"), &x.value)
    } else if "delete".eq(&x.action) {
        Flash::success(Redirect::to("/"), "")
    } else {
        Flash::error(Redirect::to("/"), &x.value)
    }
}

#[get("/<uml>")]
pub fn img(uml: &RawStr, flash: Option<FlashMessage>) -> Option<NamedFile> {
    let s =  uml.as_str();

    //let s = flash.map(|msg| format!("{}", msg.msg())).unwrap_or_else(|| "abc -> def".to_string());
    let now = time::get_time();

    let kdv: Vec<Klassendiagramm> = parser::starte_umlparser(&String::from(s)).get_klassendiagramme();
    //let usv: Vec<Usecasediagramm> = parser::starte_umlparser(&s).get_usecasediagramme();

    if kdv.get(0).is_some() {
        let kd : &Klassendiagramm = kdv.get(0).unwrap();
        let k = kd.get_klassen();
        let r = kd._get_relationen();
        for klasse in k {
            println!("Main:::Klassenname:{:?}  Koordinaten({}/{})",klasse.get_id().first(),klasse.get_pos_x(),klasse.get_pos_y());
        }
        for relation in r {
            println!("Main:::Relation:  typ:{}",relation.get_typ());
        }

        kd.get_image().save(format!("static/temp/{:?}.png", now.nsec)).unwrap();

    } else {

    }

    NamedFile::open(Path::new("static/").join(&(format!("temp/{:?}.png", now.nsec)))).ok()

}
