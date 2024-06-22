use std::env;

use svg_painter::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let raster_image_path = &args[1];

    run(raster_image_path);
}
