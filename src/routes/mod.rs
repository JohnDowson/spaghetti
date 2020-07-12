use crate::templates::*;
use maud::{html, Markup};
use rocket::response::content;
#[get("/")]
pub fn index() -> Markup {
    html! {
        div class="box" {
        (navbar(vec!["A".to_owned(), "B".to_owned(), "C".to_owned()]))
        (header("Hello, world".into()))
        div class="content" {
            p {
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Porta nibh venenatis cras sed. Arcu dictum varius duis at consectetur lorem donec massa sapien. Nisl tincidunt eget nullam non nisi. Lobortis scelerisque fermentum dui faucibus. Ultrices sagittis orci a scelerisque purus semper eget duis. Vestibulum morbi blandit cursus risus at ultrices mi tempus. Pulvinar neque laoreet suspendisse interdum consectetur libero id faucibus nisl. Vitae tortor condimentum lacinia quis vel eros donec. Eu feugiat pretium nibh ipsum consequat nisl vel pretium lectus. Ut consequat semper viverra nam libero justo. Et malesuada fames ac turpis egestas sed tempus urna. Ullamcorper malesuada proin libero nunc consequat interdum varius. Elit ullamcorper dignissim cras tincidunt."
            }
            p {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Porta nibh venenatis cras sed. Arcu dictum varius duis at consectetur lorem donec massa sapien. Nisl tincidunt eget nullam non nisi. Lobortis scelerisque fermentum dui faucibus. Ultrices sagittis orci a scelerisque purus semper eget duis. Vestibulum morbi blandit cursus risus at ultrices mi tempus. Pulvinar neque laoreet suspendisse interdum consectetur libero id faucibus nisl. Vitae tortor condimentum lacinia quis vel eros donec. Eu feugiat pretium nibh ipsum consequat nisl vel pretium lectus. Ut consequat semper viverra nam libero justo. Et malesuada fames ac turpis egestas sed tempus urna. Ullamcorper malesuada proin libero nunc consequat interdum varius. Elit ullamcorper dignissim cras tincidunt."
                }
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Porta nibh venenatis cras sed. Arcu dictum varius duis at consectetur lorem donec massa sapien. Nisl tincidunt eget nullam non nisi. Lobortis scelerisque fermentum dui faucibus. Ultrices sagittis orci a scelerisque purus semper eget duis. Vestibulum morbi blandit cursus risus at ultrices mi tempus. Pulvinar neque laoreet suspendisse interdum consectetur libero id faucibus nisl. Vitae tortor condimentum lacinia quis vel eros donec. Eu feugiat pretium nibh ipsum consequat nisl vel pretium lectus. Ut consequat semper viverra nam libero justo. Et malesuada fames ac turpis egestas sed tempus urna. Ullamcorper malesuada proin libero nunc consequat interdum varius. Elit ullamcorper dignissim cras tincidunt."
                    }
        }
        (footer())}
    }
}

#[get("/style.css", format = "text/css")]
pub fn style() -> content::Css<String> {
    use std::fs;
    content::Css(fs::read_to_string("./css/style.css").unwrap_or_default())
}
#[get("/navhider.js", format = "application/javascript")]
pub fn script() -> content::JavaScript<String> {
    use std::fs;
    content::JavaScript(fs::read_to_string("./js/navhider.js").unwrap_or_default())
}
