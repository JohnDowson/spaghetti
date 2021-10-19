use std::error::Error;

use super::public;
use crate::models::{BlogForm, BlogPost};
use crate::templates::*;

use maud::{html, Markup};

use rocket::http::Cookie;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket::Request;

use rocket::{form::Form, http::Status, State};
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct Admin {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = Box<dyn Error>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let valid = |k: &Cookie| k.value() == "valid";
        match req.cookies().get_private("session") {
            None => Outcome::Forward(()),
            Some(key) if valid(&key) => Outcome::Success(Admin {}),
            Some(_) => {
                Outcome::Failure((Status::BadRequest, Box::<dyn Error>::from("Bad API key")))
            }
        }
    }
}

#[get("/posts/<id>")]
pub async fn post(id: i64, _admin: Admin, pool: &State<SqlitePool>) -> Option<Markup> {
    match BlogPost::get(id, false, &*pool).await {
        Ok(post) => Some(page(
            &format!("hjvt::blog::{}", post.title),
            super::parse_markdown(&post.body),
        )),
        Err(_) => None,
    }
}

#[post("/posts/submit", data = "<blog>")]
pub async fn submit(
    blog: Form<BlogForm>,
    _admin: Admin,
    pool: &State<SqlitePool>,
) -> Result<Redirect, (Status, Markup)> {
    let nb = BlogPost::from_form(blog.into_inner(), false);
    match nb.commit(&*pool).await {
        Ok(id) => Ok(Redirect::to(uri!(public::post(id)))),
        Err(e) => Err((
            Status::InternalServerError,
            html! {
                (format!("Error commiting post: {:?}", e))
            },
        )),
    }
}

#[get("/posts/new")]
pub async fn new(_admin: Admin) -> Markup {
    page("hjvt::blog::new", post_editor())
}

#[delete("/posts/<id>")]
pub async fn delete_post(
    id: i64,
    _admin: Admin,
    pool: &State<SqlitePool>,
) -> Result<Redirect, (Status, Markup)> {
    BlogPost::delete(id, pool)
        .await
        .map(|_| Redirect::to(uri!(posts)))
        .map_err(|e| (Status::InternalServerError, html! { (e.to_string()) }))
}

#[post("/posts/<id>/publish")]
pub async fn publish(id: i64, _admin: Admin, pool: &State<SqlitePool>) -> Result<Status, Status> {
    BlogPost::publish(id, pool)
        .await
        .map(|_| Status::Ok)
        .map_err(|_| Status::InternalServerError)
}

#[get("/posts", rank = 1)]
pub async fn posts(_admin: Admin, pool: &State<SqlitePool>) -> Result<Markup, (Status, String)> {
    match BlogPost::all(&*pool).await {
        Ok(blogs) => Ok(page(
            "hjvt::blog",
            html! {
                @for post in blogs {
                    div {
                        a href=(uri!(public::post(post.id))) {(post.title)} {" "(post.created_at)" "}
                        button onclick=(format!("http_delete('{}')", uri!(delete_post(post.id)))) {"Delete"}
                        button onclick=(format!("http_post('{}')", uri!(publish(post.id)))) { (if post.published {"Unpublish"} else {"Publish"})}
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
