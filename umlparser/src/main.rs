#![feature(proc_macro_hygiene, decl_macro, plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

mod routes;

use rocket_contrib::Template;

fn main() {
    start_rocket();
}

fn start_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::defaultroute::index,
            ]
        ).mount(
        "/api",
        routes![
                 routes::apiroute::test,
            ]
    ).attach(Template::fairing()) // Für Templates
        .launch();
}