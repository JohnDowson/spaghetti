mod fairings;
mod models;
mod routes;
mod templates;
pub use fairings::Secrets;
use rocket::{catchers, launch, routes};
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Session {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let rocket = rocket::build();

    rocket
        .attach(fairings::DbManager)
        .attach(fairings::SecretManager)
        .mount(
            "/",
            routes![
                routes::public::index,
                routes::public::posts,
                routes::public::post,
                routes::public::new_redirect,
                routes::public::login,
                routes::public::login_post,
                routes::admin::index,
                routes::admin::delete_post,
                routes::admin::publish,
                routes::admin::post,
                routes::admin::posts,
                routes::admin::submit,
                routes::admin::new,
            ],
        )
        .mount("/static", rocket::fs::FileServer::from("./static"))
        .register(
            "/",
            catchers![
                routes::not_found_catcher,
                routes::internal_error_catcher,
                routes::unauthorized_catcher
            ],
        )
}
