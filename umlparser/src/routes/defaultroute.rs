extern crate tera;

use rocket_contrib::Template;
use tera::Context;

#[get("/")]
pub fn index() -> Template {
    let mut context = Context::new();

    context.add("my_message", &"Heya from template context!");
    Template::render("index", &context)
}
