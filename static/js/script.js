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



function highlight_current_category_in_navbar() {
    function prefix(pathname) {
        let i = 0;
        while (pathname[i] && pathname[i] == location.pathname[i])
            i++;
        return i;
    }
    var navbar_items = Array.from(document.getElementsByClassName("navbar_item")).map((it) => it.children[0]);
    navbar_items
        .sort((a, b) => prefix(b.pathname) - prefix(a.pathname));
    var current_category = navbar_items[0];
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

function http_delete(uri) {
    fetch(uri, { method: 'DELETE' })
    location.reload()
}

function http_post(uri) {
    fetch(uri, { method: 'POST' })
    location.reload()
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