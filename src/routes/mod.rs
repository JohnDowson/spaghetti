use crate::models::BlogPost;
use crate::templates::*;
use crate::DbConn;
use maud::{html, Markup};
use rocket::response::content;
use rocket::response::status::NotFound;
#[get("/")]
pub fn index() -> Markup {
    page("Hello, world".into(), html! {p {(LOREM)}})
}

fn test_markdown() -> String {
    use pulldown_cmark::{html, Options, Parser};
    use std::fs;

    let markdown_input = fs::read_to_string("./test.md").unwrap_or_default();
    let options = Options::all();
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
#[get("/posts/<id>")]
pub fn post(id: i32, conn: DbConn) -> Result<Markup, NotFound<String>> {
    match BlogPost::get(id, &conn) {
        Ok(post) => Ok(page(
            &format!("hjvt::blog::{}", post.title), // blog title goes here
            maud::PreEscaped(post.body),            // blog body goes here
        )),
        Err(_) => Err(NotFound(format!("Post id:{} not found", id))),
    }
}
#[get("/posts")]
pub fn posts() -> Markup {
    page(&"hjvt::blog", html! {p {(LOREM)}})
}
