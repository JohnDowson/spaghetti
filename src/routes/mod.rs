use crate::templates::*;
use maud::{html, Markup};
use rocket::response::content;
#[get("/")]
pub fn index() -> Markup {
    page("Hello, world".into(), html! {p {(LOREM)}})
}

#[get("/style.css", format = "text/css")]
pub fn style() -> content::Css<String> {
    use std::fs;
    content::Css(fs::read_to_string("./css/style.css").unwrap_or_default())
}
#[get("/script.js", format = "application/javascript")]
pub fn script() -> content::JavaScript<String> {
    use std::fs;
    content::JavaScript(fs::read_to_string("./js/script.js").unwrap_or_default())
}
#[get("/posts/<id>")]
pub fn post(id: usize) -> Markup {
    page(&format!("hjvt::blog::{}", id), html! {p {(LOREM)}})
}
#[get("/posts")]
pub fn posts() -> Markup {
    page(&"hjvt::blog", html! {p {(LOREM)}})
}
