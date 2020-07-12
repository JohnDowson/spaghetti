var navbarShown = true;
function navbarClick(navbar) {
    var content = Array.from(document.getElementsByClassName("box"));
    var navbar_children = Array.from(navbar.children);
    if (navbarShown == true) {
        navbar.style.width = "10";
        navbar_children.map((it) => { it.style.visibility = "hidden" });
        content.map((it) => { console.log("Resizing content"); it.style.margin = '0 10% 0 10%' });
        navbarShown = false
    } else {
        navbar.style.width = "200px";
        navbar_children.map((it) => { it.style.visibility = "visible" });
        content.map((it) => { console.log("Restoring content"); it.style.margin = '0 10% 0 240px' });
        navbarShown = true
    }
}
function navbarHover() {
    var tooltip = document.getElementsByClassName("tooltiptext")[0];
    tooltip.style.visibility = "visible";
    tooltip.textContent = navbarShown ? "Click to hide" : "Click to show";
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