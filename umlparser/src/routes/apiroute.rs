extern crate tera;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use tera::Context;

#[derive(FromForm)]
struct Eingabe {
    value: String
}

#[get("/")]
pub fn index(flash: Option<FlashMessage>) -> Template {
    let mut context = Context::new();

    let s = flash.map(|msg| format!("{}", msg.msg())).unwrap_or_else(|| "Normaler Text!".to_string());

    context.add("my_message", &s);
    Template::render("content", &context)
}

#[post("/submit", data = "<eingabe>")]
fn submit_task(eingabe: Form<Eingabe>) -> Flash<Redirect> {
    let x: Eingabe = eingabe.into_inner();

    Flash::success(Redirect::to("/"), &x.value)
}