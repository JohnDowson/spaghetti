document.addEventListener('DOMContentLoaded', () => {
    if (sessionStorage.getItem("navbarShown") == null) {
        sessionStorage.setItem("navbarShown", true);
    }
    navbar_set_state(sessionStorage.getItem("navbarShown") == "false")
    document.getElementById("navbar").addEventListener('click', navbarClick);
    highlight_current_category_in_navbar()
    document.querySelectorAll('pre code').forEach((block) => {
        hljs.highlightBlock(block);
    });
})
function pathname_from(href) {
    var url = href.split("/");
    return url[url.length - 2]
}
function highlight_current_category_in_navbar() {
    var navbar_items = Array.from(document.getElementsByClassName("navbar_item")).map((it) => it.children[0]);
    var current_category = navbar_items.find((it) => pathname_from(it.href) == pathname_from(location.href));
    current_category.textContent = "> " + current_category.textContent + " <"
    current_category.style.fontWeight = "bold";
}
function navbarClick(event) {
    var target_is_tooltip = document.getElementsByClassName("tooltiptext")[0] === event.target;
    if (event.target !== this) { if (!target_is_tooltip) { return } }

    if (sessionStorage.navbarShown == "true") {
        navbar_set_state(true)
        sessionStorage.setItem("navbarShown", false);
    } else {
        navbar_set_state(false)
        sessionStorage.setItem("navbarShown", true);
    }
}
function navbar_set_state(hidden) {
    var content = Array.from(document.getElementsByClassName("box"));
    var navbar_children = Array.from(navbar.children);
    if (hidden === true) {
        navbar.style.width = "10px";
        navbar_children.map((it) => { it.style.visibility = "hidden" });
        content.map((it) => { it.style.margin = '0 10% 0 10%' });

    } else if (hidden === false) {
        navbar.style.width = "200px";
        navbar_children.map((it) => { it.style.visibility = "visible" });
        content.map((it) => { it.style.margin = '0 10% 0 240px' });

    }
}

function navbarHover() {
    var tooltip = document.getElementsByClassName("tooltiptext")[0];
    tooltip.style.visibility = "visible";
    tooltip.textContent = (sessionStorage.navbarShown == "true") ? "Click to hide" : "Click to show";
    window.onmousemove = function (e) {
        var x = e.clientX,
            y = e.clientY;
        tooltip.style.top = (y) + 'px';
        tooltip.style.left = (x) + 'px';
    };

    setTimeout(() => {
        tooltip.style.visibility = "hidden";
        window.onmousemove = null;
    }, 1000);
}