#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
mod models;
mod routes;
mod templates;
use diesel::SqliteConnection;
use rocket_contrib::serve::StaticFiles;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);
fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/css", StaticFiles::from("./css"))
        .mount("/js", StaticFiles::from("./js"))
        .mount("/", routes![routes::index, routes::posts, routes::post])
        .launch();
}
