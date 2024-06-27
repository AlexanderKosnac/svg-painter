# svg-painter

Create an SVG based image approximation of a raster-based image.

## Build and Run

The project can be built and ran via the respective cargo commands:

```bash
cargo build
cargo run -- <path_to_png>
```

The image to approximate is the first and only argument.
At this point only PNGs can be used.

To compile it to WebAssembly, run:

```bash
wasm-pack build --target web
```


## How it works

The approximation is achieved via a strongly-guided evolutionary algorithm.
For this, the approximation is assembled progressively by adding strokes.
To add a stroke, a random position inside the image is chosen as a start point.
From here a number of attempts is made to improve the results of this stroke, by slightly shifting its position and rotation.
This continues until no improvements can be gained for multiple attempts.
The color of the stroke is always set to the average color in the area that the stroke covers in the target image.

To evaluate the fitness, the square norm is calculated over the target image and approximation.
For this, the vector based approximation is rendered into a raster based format.

After failing to insert multiple strokes in a row, the stroke size is decreased to achieve finer details.
