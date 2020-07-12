#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod models;
mod routes;
mod templates;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::style,
                routes::script,
                routes::posts,
                routes::post
            ],
        )
        .launch();
}
