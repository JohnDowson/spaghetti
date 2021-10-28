use maud::{html, Markup, DOCTYPE};
pub fn post_editor(form_action: &str) -> Markup {
    html! {
        form action=(form_action) method="post" id="post_form" {
            "Title:";
            input type="text" name="title";
            input type="submit";
        }
        textarea name="body" form="post_form" {};
    }
}

pub fn page(page_title: &str, content: Markup) -> Markup {
    html! {
        (html_head(page_title))
        div class="box" {
        (navbar(&[
            ("About", "/"),
            ("Blog", "/posts/"),
            ("Github", "https://github.com/JohnDowson"),
        ]))
        (body(content, page_title))
        (footer())}
    }
}

pub fn admin_page(page_title: &str, content: Markup) -> Markup {
    html! {
        (html_head(page_title))
        div class="box" {
        (navbar(&[
            ("About", "/"),
            ("Blog", "/posts/"),
            ("New", "/posts/new"),
            ("Logout","/logout"),
            ("Github", "https://github.com/JohnDowson")
        ]))
        (body(content, page_title))
        (footer())}
    }
}

pub fn html_head(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        // Favicon magic
        link rel="icon" href="/static/favicon.ico" type="image/x-icon";
        link rel="apple-touch-icon" sizes="120x120" href="/static/apple-touch-icon.png" {}
        link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32x32.png" {}
        link rel="icon" type="image/png" sizes="16x16" href="/static/favicon-16x16.png" {}
        link rel="manifest" href="/static/site.webmanifest" {}
        link rel="mask-icon" href="/static/safari-pinned-tab.svg" color="#007f46" {}
        link rel="shortcut icon" href="/static/favicon.ico" {}
        meta name="msapplication-TileColor" content="#da532c" {}
        meta name="msapplication-config" content="/static/browserconfig.xml" {}
        meta name="theme-color" content="#007f46" {}
        // Favicon magic
        meta charset="utf-8";
        title { (page_title) }
        link rel="stylesheet" type="text/css" href="/static/css/style.css" ;
        link rel="stylesheet" type="text/css" href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/styles/srcery.min.css" ;
        script src="/static/js/script.js" {}
        script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/highlight.min.js" {}
    }
}
pub fn body(content: Markup, page_title: &str) -> Markup {
    html! {
        header id="header" {
            h1 { (page_title) }
        }
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
pub fn navbar(items: &[(&str, &str)]) -> Markup {
    html! {
        div id="navbar" class="navbar" onmouseover="navbarHover()" {
            h2 {("hjvt::*")}
            @for (item, link_to) in items {
                (navbar_item(item, link_to))
            }
            span class="tooltiptext" {}
        }
    }
}
pub fn navbar_item(item: &str, link_to: &str) -> Markup {
    html! { p class="navbar_item" { a href=(link_to) {(item)} } }
}
