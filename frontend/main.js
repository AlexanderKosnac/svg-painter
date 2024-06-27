import init from "../pkg/svg_painter.js";
init().then(() => {
    let target = document.querySelector("#target");
    let svgContainer = document.querySelector("#svg-container");
    let runButton = document.querySelector("#start");
    let terminateButton = document.querySelector("#terminate");
    let fileInput = document.querySelector("#input-file");

    function loadImage() {
        if (fileInput.files.length < 1) return;
        const file = fileInput.files[0];
        const reader = new FileReader();
        reader.onload = function(e) {
            const img = new Image();
            img.onload = function() {
                const ctx = target.getContext("2d");

                target.width = img.width;
                target.height = img.height;
                target.style.width = img.width;
                target.style.height = img.height;
                target.style.minWidth = img.width;

                ctx.drawImage(img, 0, 0);
            };
            img.src = e.target.result;
        };
        reader.readAsDataURL(file);
    }

    let worker;

    runButton.addEventListener("click", () => {
        const ctx = target.getContext("2d");
        const data = ctx.getImageData(0, 0, target.width, target.height).data;

        worker = new Worker("worker.js", { type: "module" });

        worker.onmessage = (e) => {
            switch(e.data[0]) {
                case "SVG":
                    svgContainer.innerHTML = e.data[1];
                    break;
                case "DONE":
                    break;
                default:
                    console.error("Unknown message type");
            }
        };

        worker.postMessage({ pxdata: data, width: target.width, height: target.height });
    });

    terminateButton.addEventListener("click", () => {
        worker.terminate();
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
