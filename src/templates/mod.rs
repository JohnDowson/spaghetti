use maud::{html, Markup, DOCTYPE};
pub const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn page(page_title: &str, content: Markup) -> Markup {
    html! {
        div class="box" {
        (navbar(vec![
            ("Github", "https://github.com/JohnDowson"),
        ("Blog", "/posts/"),
        ]))
        (header(page_title))
        (body(content))
        (footer())}
    }
}
pub fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        script src="/script.js" {}
        link rel="stylesheet" type="text/css" href="/style.css" /
        meta charset="utf-8";
        title { (page_title) }
        header id="header" {
            h1 { (page_title) }
        }
    }
}
pub fn body(content: Markup) -> Markup {
    html! {
        div class="content" {
            (content)
        }
    }
}
pub fn footer() -> Markup {
    html! {
        footer {
            p { "2020, hjvt©" }
        }
    }
}
pub fn navbar(items: Vec<(&str, &str)>) -> Markup {
    html! {
        div class="navbar" onclick="navbarClick(this)" onmouseover="navbarHover()" {
            h2 {("hjvt::*")}
            @for (item, link_to) in items {
                (navbar_item(item.into(), link_to.into()))
            }
            span class="tooltiptext" {}
        }
    }
}
pub fn navbar_item(item: String, link_to: String) -> Markup {
    html! { p class="navbar_item" { a href=(link_to) {(item)} } }
}
