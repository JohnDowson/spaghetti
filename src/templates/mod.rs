use maud::{html, Markup, DOCTYPE};
pub const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn post_editor() -> Markup {
    html! {
        header id="header" {
            form style="text-align: center;" {
            input style="
                color: #fecc80;
                border-style: inset;
                border-width: 2px;
                font-style: bold;
                font-size: 2em;
                font-family: 'Inconsolata', monospace;
                font-weight: 500;
                background:none;
                vertical-align: text-bottom;"
                type="text" id="post_title";
            button style="
                position: relative;
                box-sizing:border-box;
                background-color: #528623;
                border: none;
                color: #fdc168;
                text-align: center;
                text-decoration: none;
                display: inline-block;
                font-size: 16px;
                vertical-align: text-bottom;
                margin-top: 10px;
                margin-right: 30px;
                margin-bottom: 2px;
                padding-top: 10px;
                padding-right: 10px;
                padding-bottom: 10px;
                padding-left: 10px;
                cursor: pointer;"
                type="button" onclick="alert('Hello World!')" {"Submit"}}
        }
        div class="content" {
            textarea style="
            box-sizing:border-box;
            width:100%;
            height:100%;
            resize:none;
            color: #fecc80;
            font-weight: 350;
            font-family: 'Josefin Sans', sans-serif;
            font-size: 13pt;
            background:none"
            id="post_body" {}
        }
    }
}

pub fn page(page_title: &str, content: Markup) -> Markup {
    html! {
        (html_head(page_title))
        div class="box" {
        (navbar(vec![
            ("Index", "/"),
        ("Blog", "/posts/"),("Github", "https://github.com/JohnDowson"),
        ]))
        (body(content, page_title))
        (footer())}
    }
}
pub fn html_head(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        // Favicon magic
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
pub fn navbar(items: Vec<(&str, &str)>) -> Markup {
    html! {
        div id="navbar" class="navbar" onmouseover="navbarHover()" {
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
