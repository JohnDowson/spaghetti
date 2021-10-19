mod models;
mod routes;
mod templates;
use std::env::var;
#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let db_uri = var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db/dev.sqlite3".to_string());
    let pool = sqlx::SqlitePool::connect(&db_uri)
        .await
        .expect("Couldn't create DB pool");

    rocket::build()
        .mount(
            "/",
            rocket::routes![
                routes::public::index,
                routes::public::posts,
                routes::admin::posts,
                routes::public::post,
                routes::admin::submit,
                routes::admin::new,
                routes::public::new_redirect,
                routes::public::login,
                routes::public::login_post,
                routes::admin::delete_post,
                routes::admin::publish,
                routes::admin::post,
            ],
        )
        .mount(
            "/static",
            rocket::fs::FileServer::from(rocket::fs::relative!("/static")),
        )
        .register("/", catchers![routes::not_found_catcher])
        .manage(pool)
}
