document.addEventListener('DOMContentLoaded', () => {
    highlight_current_category_in_navbar();
})



function highlight_current_category_in_navbar() {
    function prefix(pathname) {
        let i = 0;
        let eq = true;
        while (true) {
            if (!location.pathname[i]) { break; }
            if (eq && !pathname[i]) { break; }
            eq = pathname[i] == location.pathname[i];
            i++;
        }
        return eq;
    }
    var navbar_items = Array.from(document.getElementsByClassName("navbar_item")).map((it) => it.children[0]);
    let current_category = navbar_items
        .find((a) => prefix(a.pathname));
    current_category.textContent = "> " + current_category.textContent + " <"
    current_category.style.fontWeight = "bold";
}

function http_delete(uri) {
    fetch(uri, { method: 'DELETE' })
    location.reload()
}

function http_post(uri) {
    fetch(uri, { method: 'POST' })
    location.reload()
}
