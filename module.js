export function js_print(text) {
    console.log(text);
}

export function set_svg(svg) {
    document.querySelector("div#svg-approximation").innerHTML = svg;
}

export function set_target(url) {
    document.querySelector("img#target").src = url;
}
