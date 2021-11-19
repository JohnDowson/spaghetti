pub mod admin;
pub mod public;
use maud::{html, Markup};
use rocket::{
    catch, get,
    http::Status,
    response::{status, Redirect},
    Request,
};

use crate::templates::page;

fn parse_markdown(markdown: &str) -> Markup {
    use pulldown_cmark::{html, Options, Parser};
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    maud::PreEscaped(html_output)
}

fn error(e: Box<dyn std::error::Error>) -> Status {
    log::error!("{}", e);
    Status::InternalServerError
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/about")
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

#[catch(500)]
pub fn internal_error_catcher(_req: &Request<'_>) -> status::Custom<Markup> {
    status::Custom(
        Status::InternalServerError,
        page(
            "500",
            html! {
                h3 {"Internal Server Error"}
            },
        ),
    )
}

#[catch(401)]
pub fn unauthorized_catcher(_req: &Request<'_>) -> status::Custom<Markup> {
    status::Custom(Status::Unauthorized, page("Gtfo", html! {}))
}
