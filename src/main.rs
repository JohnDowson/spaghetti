mod fairings;
mod models;
mod routes;
mod templates;
#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let rocket = rocket::build();

    rocket
        .attach(fairings::DbManager)
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
        .mount("/static", rocket::fs::FileServer::from("./static"))
        .register(
            "/",
            catchers![routes::not_found_catcher, routes::internal_error_catcher],
        )
}
