use maud::{html, Markup, PreEscaped, DOCTYPE};
pub fn header(page_title: String) -> Markup {
    html! {
        (DOCTYPE)
        script src="navhider.js" {}
        link rel="stylesheet" type="text/css" href="style.css" /
        meta charset="utf-8";
        title { (page_title) }
        header {
            h1 { (page_title) }
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
pub fn navbar(items: Vec<String>) -> Markup {
    html! {
        div class="navbar" onclick="myFunction()" {
            h2 {"Navbar"}
        @for item in items {
            (navbar_item(item))
        }
    }
    }
}
pub fn navbar_item(item: String) -> Markup {
    html! { p class="navbar_item" { (item) } }
}
