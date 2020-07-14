#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
mod fairings;
mod models;
mod routes;
mod templates;
use diesel::SqliteConnection;
use fairings::TimeRequests;
use rocket_contrib::serve::StaticFiles;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(TimeRequests)
        .attach(DbConn::fairing())
        .mount("/static", StaticFiles::from("./static"))
        .mount("/", routes![routes::index, routes::posts, routes::post])
        .launch();
}
