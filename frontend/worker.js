import init, { run_js } from "../pkg/svg_painter.js";
onmessage = (e) => {
    init().then(() => {
        const data = e.data;
        run_js(data.pxdata.join(","), data.width, data.height);
        postMessage(["DONE"]);
    });
};
