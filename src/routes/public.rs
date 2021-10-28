use crate::models::{about, BlogPost};
use crate::routes::error;
use crate::{templates::*, Secrets, Session};
use chrono::{Duration, Utc};
use jwt::SignWithKey;
use maud::{html, Markup};
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::{form::Form, get, http::Status, post, uri, FromForm, State};
use sqlx::PgPool;

#[get("/", rank = 2)]
pub async fn index(pool: &State<PgPool>) -> Result<Markup, Status> {
    about(&*pool)
        .await
        .map(|about| admin_page("hjvt::about", super::parse_markdown(&about)))
        .map_err(|e| error(e))
}

#[get("/posts/<id>", rank = 2)]
pub async fn post(id: i32, pool: &State<PgPool>) -> Option<Markup> {
    match BlogPost::get(id, true, &*pool).await {
        Ok(post) => Some(page(
            &format!("hjvt::blog::{}", post.title),
            super::parse_markdown(&post.body),
        )),
        Err(_) => None,
    }
}

#[get("/posts", rank = 2)]
pub async fn posts(pool: &State<PgPool>) -> Result<Markup, Status> {
    match BlogPost::all_published(&*pool).await {
        Ok(blogs) => Ok(page(
            "hjvt::blog",
            html! {
                table class="blogs" {
                @for post in blogs {
                    tr {
                        td { a href=(uri!(post(post.id))) {(post.title)}}
                        td class="posted_at" {(post.created_at.format("%Y/%m/%d %H:%M"))}
                        br;
                    }
                }}
            },
        )),
        Err(e) => Err(error(e)),
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
pub fn login_post(
    login: Form<Login>,
    cookies: &CookieJar,
    secret: &State<Secrets>,
) -> Result<Markup, Status> {
    let now = Utc::now();
    let claims = Session {
        sub: String::from("Admin"),
        iat: now.timestamp(),
        exp: (now + Duration::weeks(1)).timestamp(),
    };
    let token_str = claims.sign_with_key(secret.secret_key()).unwrap();
    if bcrypt::verify(&login.password, secret.admin_password()).map_err(|e| error(e.into()))? {
        cookies.add_private(Cookie::new("session", token_str));
        Ok(page("Logged in", html! {}))
    } else {
        Err(Status::Unauthorized)
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
