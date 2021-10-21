mod models;
mod routes;
mod templates;
#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let rocket = rocket::build();
    let db_uri: String = rocket
        .figment()
        .extract_inner("db")
        .expect("Please configure ROCKET_DB");
    log::info!("Using db: {}", db_uri);
    let pool = sqlx::PgPool::connect(&db_uri)
        .await
        .expect("Couldn't create DB pool");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Couldn't run migrations");

    rocket
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
        .manage(pool)
}
