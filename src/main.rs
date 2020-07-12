#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod routes;
mod templates;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::index, routes::style, routes::script])
        .launch();
}
