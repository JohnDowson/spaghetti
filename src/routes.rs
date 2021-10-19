pub mod admin;
pub mod public;
use maud::{html, Markup};
use rocket::{response::status, Request};

use crate::templates::page;

fn parse_markdown(markdown: &str) -> Markup {
    use pulldown_cmark::{html, Options, Parser};
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    maud::PreEscaped(html_output)
}

#[catch(404)]
pub fn not_found_catcher(req: &Request<'_>) -> status::NotFound<Markup> {
    status::NotFound(page(
        "404",
        html! {
            h3 {(format!("{} not found", req.uri()))}
        },
    ))
}
