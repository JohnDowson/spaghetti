use super::public;
use crate::models::{about, BlogForm, BlogPost};
use crate::routes::error;
use crate::{templates::*, Secrets, Session};

use chrono::{DateTime, NaiveDateTime, Utc};
use jwt::VerifyWithKey;
use maud::{html, Markup};

use rocket::http::Cookie;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket::Request;

use rocket::{delete, form::Form, get, http::Status, post, uri, State};
use sqlx::PgPool;

#[derive(Debug)]
pub struct Admin;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let secret = req.rocket().state::<Secrets>().unwrap();
        let valid = |k: &Cookie| -> bool {
            let claims: Session = k.value().verify_with_key(secret.secret_key()).unwrap();
            let exp = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(claims.exp, 0), Utc);
            let now = Utc::now();
            let expired = exp < now;
            log::info!("Now:{}, exp: {}", now, exp);
            !expired && claims.sub == "Admin"
        };

        match req.cookies().get_private("session") {
            Some(key) if valid(&key) => Outcome::Success(Admin),
            _ => Outcome::Forward(()),
        }
    }
}

#[get("/")]
pub async fn index(_admin: Admin, pool: &State<PgPool>) -> Result<Markup, Status> {
    about(&*pool)
        .await
        .map(|about| admin_page("hjvt::about", super::parse_markdown(&about)))
        .map_err(|e| error(e))
}

#[get("/posts/<id>")]
pub async fn post(id: i32, _admin: Admin, pool: &State<PgPool>) -> Option<Markup> {
    match BlogPost::get(id, false, &*pool).await {
        Ok(post) => Some(admin_page(
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
    pool: &State<PgPool>,
) -> Result<Redirect, Status> {
    let nb = BlogPost::from_form(blog.into_inner(), false);
    match nb.commit(&*pool).await {
        Ok(id) => Ok(Redirect::to(uri!(public::post(id)))),
        Err(e) => Err(error(e)),
    }
}

#[get("/posts/new")]
pub async fn new(_admin: Admin) -> Markup {
    admin_page("hjvt::blog::new", post_editor())
}

#[delete("/posts/<id>")]
pub async fn delete_post(id: i32, _admin: Admin, pool: &State<PgPool>) -> Result<Redirect, Status> {
    BlogPost::delete(id, pool)
        .await
        .map(|_| Redirect::to(uri!(posts)))
        .map_err(|e| error(e))
}

#[post("/posts/<id>/publish")]
pub async fn publish(id: i32, _admin: Admin, pool: &State<PgPool>) -> Result<Status, Status> {
    BlogPost::publish(id, pool)
        .await
        .map(|_| Status::Ok)
        .map_err(|_| Status::InternalServerError)
}

#[get("/posts", rank = 1)]
pub async fn posts(_admin: Admin, pool: &State<PgPool>) -> Result<Markup, Status> {
    match BlogPost::all(&*pool).await {
        Ok(blogs) => Ok(admin_page(
            "hjvt::blog",
            html! {
                table class="blogs" {
                @for post in blogs {
                    tr {
                        td { a href=(uri!(public::post(post.id))) {(post.title)}}
                        td class="posted_at" {(post.created_at.format("%Y/%m/%d %H:%M"))}
                        td {
                            button onclick=(format!("http_delete('{}')", uri!(delete_post(post.id)))) {"Delete"}
                            button onclick=(format!("http_post('{}')", uri!(publish(post.id)))) { (if post.published {"Unpublish"} else {"Publish"})}
                        }
                        br;
                    }
                }}
            },
        )),
        Err(e) => Err(error(e)),
    }
}
