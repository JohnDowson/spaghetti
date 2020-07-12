var state = true;
function myFunction() {
    var content = Array.from(document.getElementsByClassName("box"));
    var navbar = document.getElementsByClassName("navbar")[0];
    var navbar_children = Array.from(navbar.children);
    if (state == true) {
        navbar.style.width = "10";
        navbar_children.map((it) => { it.style.visibility = "hidden" });
        content.map((it) => { it.style.margin_left = "10%" });
        state = false
    } else {
        navbar.style.width = "200px";
        navbar_children.map((it) => { it.style.visibility = "visible" });
        content.map((it) => { it.style.margin_left = "240px" });
        state = true
    }
}