pub mod admin;
pub mod public;

use maud::{html, Markup};
use rocket::{
    catch, get,
    http::Status,
    response::{status, Redirect},
    Request,
};

use extrusion_dies::templates::page;

fn parse_markdown(markdown: &str) -> Markup {
    use comrak::{
        markdown_to_html_with_plugins, plugins::syntect::SyntectAdapterBuilder, Options, Plugins,
    };
    let adapter = SyntectAdapterBuilder::new().css().build();
    let options = Options::default();
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let html_output = markdown_to_html_with_plugins(markdown, &options, &plugins);
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
        &[
            ("About", "/about"),
            ("Contacts", "/contacts"),
            ("Blog", "/posts/"),
            ("Github", "https://github.com/JohnDowson"),
        ],
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
            &[
                ("About", "/about"),
                ("Contacts", "/contacts"),
                ("Blog", "/posts/"),
                ("Github", "https://github.com/JohnDowson"),
            ],
            html! {
                h3 {"Internal Server Error"}
            },
        ),
    )
}

#[catch(401)]
pub fn unauthorized_catcher(_req: &Request<'_>) -> status::Custom<Markup> {
    status::Custom(
        Status::Unauthorized,
        page(
            "Gtfo",
            &[
                ("About", "/about"),
                ("Contacts", "/contacts"),
                ("Blog", "/posts/"),
                ("Github", "https://github.com/JohnDowson"),
            ],
            html! {},
        ),
    )
}
