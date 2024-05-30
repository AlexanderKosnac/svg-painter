use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

pub mod genetic;
use genetic::*;
use genetic::svg::*;

pub mod util;

static BUILD: &str = "build";

fn main() {
    let args: Vec<String> = env::args().collect();

    let raster_image_path = &args[1];

    let genome_size = 0;
    let population_size = 50;

    fs::create_dir_all(String::from(BUILD)).expect("Unable to create build directory");
    fs::copy(raster_image_path, format!("{BUILD}/trgt.png")).expect("Could not copy target file");

    let mut env = Experiment::<SvgElementGenome<StrokeBase>>::new(raster_image_path, population_size, genome_size);
    env.evolve();
}
