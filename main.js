import init from "./pkg/svg_painter.js";
init().then(() => {
    let targetImg = document.querySelector("#target");
    let targetCanvas = document.querySelector("#target-canvas");
    let svgApprox = document.querySelector("#svg-approximation");
    let runButton = document.querySelector("#start");
    let fileInput = document.querySelector("#input-file");

    function loadImage() {
        if (fileInput.files.length < 1) return;
        const file = fileInput.files[0];
        const url = URL.createObjectURL(file);
        targetImg.src = url;
    }

    runButton.addEventListener("click", () => {
        const ctx = targetCanvas.getContext("2d");

        let dim = [targetImg.width, targetImg.height];
        [targetCanvas.width, targetCanvas.height] = dim;

        ctx.drawImage(target, 0, 0);
        const data = ctx.getImageData(0, 0, dim[0], dim[1]).data;

        const worker = new Worker("worker.js", { type: "module" });

        worker.onmessage = (e) => {
            switch(e.data[0]) {
                case "SVG":
                    svgApprox.innerHTML = e.data[1];
                    break;
                case "DONE":
                    break;
                default:
                    console.error("Unknown message type");
            }
        };

        worker.postMessage({ pxdata: data, width: dim[0], height: dim[1] });
    });

    fileInput.addEventListener("change", loadImage);

    if (fileInput.files.length > 0) loadImage();

    document.querySelector("#download").addEventListener("click", () => {
        let element = document.createElement("a");
        element.setAttribute("href", "data:text/plain;charset=utf-8," + encodeURIComponent(svgContainer.innerHTML));
        element.setAttribute("download", `svg-painting-${new Date().getTime()}.svg`);
        element.style.display = "none";
        document.body.appendChild(element);
        element.click();
        document.body.removeChild(element);
    });
});
