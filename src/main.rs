use std::env;

use svg_painter::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let output_path = &args[1];
    let raster_image_path = &args[2];

    run(output_path, raster_image_path);
}
