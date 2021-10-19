use crate::models::BlogPost;
use crate::templates::*;
use maud::{html, Markup};
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::{form::Form, http::Status, State};
use sqlx::SqlitePool;

#[get("/")]
pub fn index() -> Markup {
    page("hjvt::about", maud::PreEscaped(LOREM.into()))
}

#[get("/posts/<id>", rank = 2)]
pub async fn post(id: i64, pool: &State<SqlitePool>) -> Option<Markup> {
    match BlogPost::get(id, true, &*pool).await {
        Ok(post) => Some(page(
            &format!("hjvt::blog::{}", post.title),
            super::parse_markdown(&post.body),
        )),
        Err(_) => None,
    }
}

#[get("/posts", rank = 2)]
pub async fn posts(pool: &State<SqlitePool>) -> Result<Markup, (Status, String)> {
    match BlogPost::all_published(&*pool).await {
        Ok(blogs) => Ok(page(
            "hjvt::blog",
            html! {
                @for post in blogs {
                    div {
                        a href=(uri!(post(post.id))) {(post.title)} {" "(post.created_at)}
                        br;
                    }
                }
            },
        )),
        Err(e) => Err((
            Status::InternalServerError,
            format!("Error retrieving posts: {:?}", e),
        )),
    }
}

#[get("/login")]
pub fn login() -> Markup {
    page(
        "hjvt::login",
        html! {
            form action="/login" method="post" id="loginform" {
                input type="password" name="password";
            }
        },
    )
}

#[post("/login", data = "<login>")]
pub fn login_post(login: Form<Login>, cookies: &CookieJar) -> Markup {
    if &login.password == "Foobar" {
        cookies.add_private(Cookie::new("session", "valid".to_string()));
        page("Logged in", html! {})
    } else {
        page("Gtfo", html! {})
    }
}

#[derive(Debug, FromForm)]
pub struct Login {
    password: String,
}

#[get("/posts/new", rank = 1)]
pub fn new_redirect() -> Redirect {
    Redirect::to(rocket::uri!(login))
}
