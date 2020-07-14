use crate::models::BlogPost;
use crate::templates::*;
use crate::DbConn;
use maud::{html, Markup};
use rocket::response::{status, Redirect};
#[get("/")]
pub fn index() -> Markup {
    page("Hello, world".into(), maud::PreEscaped(LOREM.into()))
}

#[allow(dead_code)]
fn test_markdown() -> String {
    use pulldown_cmark::{html, Options, Parser};
    use std::fs;

    let markdown_input = fs::read_to_string("./test.md").unwrap_or_default();
    let options = Options::all();
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    markdown_input
}

fn parse_markdown(markdown: &str) -> Markup {
    use pulldown_cmark::{html, Options, Parser};
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    maud::PreEscaped(html_output)
}

#[get("/posts/<id>")]
pub fn post(id: i32, conn: DbConn) -> Result<Markup, status::NotFound<String>> {
    match BlogPost::get(id, &conn) {
        Ok(post) => Ok(page(
            &format!("hjvt::blog::{}", post.title),
            parse_markdown(&post.body),
        )),
        Err(_) => Err(status::NotFound(format!("Post id:{} not found", id))),
    }
}
#[get("/posts")]
pub fn posts(conn: DbConn) -> Result<Markup, status::Custom<String>> {
    match BlogPost::all(&conn) {
        Ok(blogs) => Ok(page(
            &"hjvt::blog",
            html! {
                ul {
                    @for post in blogs {
                        li { div { a href=(format!("/posts/{}", post.id)) {(post.title)} br {(post.created)} }   }
                    }
                }
            },
        )),
        Err(e) => Err(status::Custom(
            rocket::http::Status::raw(500),
            format!("Error retrieving posts: {:?}", e),
        )),
    }
}
#[get("/posts/new")]
pub fn new() -> Markup {
    page(&format!("hjvt::blog::new"), post_editor())
}
#[get("/posts/test_insert")]
pub fn insert_test(conn: DbConn) -> Result<Markup, status::Custom<String>> {
    let nb = BlogPost::new("Hello2", &test_markdown(), true);
    match nb.commit(&conn) {
        Ok(id) => Ok(page(
            &"hjvt::blog",
            html! {
            a href={("/posts/")(id)} {("post id:")(id)(" created")} },
        )),
        Err(e) => Err(status::Custom(
            rocket::http::Status::raw(500),
            format!("Error commiting post: {:?}", e),
        )),
    }
}
