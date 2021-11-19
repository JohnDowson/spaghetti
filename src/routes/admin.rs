use super::public;
use crate::models::{get_info, list_info_kinds, set_info, BlogForm, BlogPost};
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
#[derive(Debug)]
pub struct RevokeSession;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RevokeSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        req.cookies().remove_private(Cookie::named("session"));
        Outcome::Success(RevokeSession)
    }
}

#[get("/about")]
pub async fn index(_admin: Admin, pool: &State<PgPool>) -> Result<Markup, Status> {
    get_info("about", &*pool)
        .await
        .map(|about| admin_page("hjvt::about", super::parse_markdown(&about)))
        .map_err(|e| error(e))
}

#[get("/contacts")]
pub async fn contacts(_admin: Admin, pool: &State<PgPool>) -> Result<Markup, Status> {
    get_info("contacts", &*pool)
        .await
        .map(|about| admin_page("hjvt::contacts", super::parse_markdown(&about)))
        .map_err(|e| error(e))
}

#[get("/logout")]
pub async fn logout(_revoke: RevokeSession) -> Redirect {
    Redirect::to("/")
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
    admin_page("hjvt::blog::new", post_editor("/posts/submit"))
}

#[post("/admin/info", data = "<info>")]
pub async fn submit_info(
    info: Form<BlogForm>,
    _admin: Admin,
    pool: &State<PgPool>,
) -> Result<Redirect, Status> {
    match set_info(&info.body, &info.title, &*pool).await {
        Ok(_) => Ok(Redirect::to("/admin/info/new")),
        Err(e) => Err(error(e)),
    }
}

#[get("/admin/info/new")]
pub async fn new_info(_admin: Admin, pool: &State<PgPool>) -> Result<Markup, Status> {
    let info_kinds = list_info_kinds(&*pool).await.map_err(error)?;
    Ok(admin_page("THE_BACKROOMS::boo", info_editor(&info_kinds)))
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
