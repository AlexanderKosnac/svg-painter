use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

use image::{GenericImageView, Rgba};

use resvg;
use usvg;
use fontdb;
use tiny_skia;
use tiny_skia_path;

pub mod genetic;
use genetic::CircleGenome;

pub mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    let raster_image_path = &args[1];
    let n_generations = 10000000;

    let genome_size = 500;
    let population_size = 50;

    evolve(raster_image_path, n_generations, genome_size, population_size);
}

fn evolve(raster_image_path: &String, n_generations: u64, genome_size: u32, population_size: u64) {
    let target = read_image(raster_image_path);

    let mut population: Vec<(CircleGenome, f64)> = (0..population_size).map(|_| (CircleGenome::new(genome_size, target.width, target.height), 0.0)).collect();

    let mut generation: u64 = 0;
    loop {
        generation += 1;
        println!("Generation {}", generation);
        let dir = format!("build/generation_{:0>3}", generation);
        fs::create_dir_all(&dir).expect("Unable to create generation directory");

        let mut i = 0;
        for individual in &mut population {
            let base = format!("{}/genome_expression_{:0>4}", dir, i);
            let svg_data = individual.0.express();
            let mut f = File::create(format!("{base}.svg")).expect("Unable to create file");
            f.write_all(svg_data.as_bytes()).expect("Unable to write data");
            store_svg_as_png(format!("{base}.png"), &svg_data, target.width, target.height);
            let candidate_image = read_image(&format!("{base}.png"));
            individual.1 = candidate_image.distance(&target);
            i += 1;
        }

        population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let fittest = &population[0..5];

        for individual in population.iter().take(5) {
            println!("Individual: fitness {:.2}; Genome Size: {}", individual.1, individual.0.len());
        }

        if generation == n_generations {
            break;
        }

        population = setup_population(fittest, population_size);
    }
}

fn setup_population(base_individuals: &[(CircleGenome, f64)], population_size: u64) -> Vec<(CircleGenome, f64)> {
    let mut population: Vec<(CircleGenome, f64)> = Vec::new();

    let individuals_per_genome = population_size / (base_individuals.len() as u64);
    for individual in base_individuals {
        for _ in 0..individuals_per_genome {
            let mut new_genome = individual.0.clone();
            new_genome.mutate();
            population.push((new_genome, 0.0));
        }
    }

    return population;
}

struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Rgba<u8>>>,
}

impl Image {
    fn distance(&self, other: &Image) -> f64 {
        if self.width != other.width || self.height != other.height {
            panic!("Images of different dimensions can not be compared. Got {}x{} and {}x{}.", self.width, self.height, other.width, other.height);
        }
        let mut norm: f64 = 0.0;
        for i in 0..(self.width as usize) {
            for j in 0..(self.height as usize) {
                let mut vec_norm: u64 = 0;
                for k in 0..3 {
                    let v = abs_diff(self.pixels[j][i].0[k], other.pixels[j][i].0[k]);
                    vec_norm += (v as u64 * v as u64).pow(2);
                }
                norm += (vec_norm as f64).sqrt();
            }
        }
        return (norm as f64).sqrt() as f64;
    }
}

fn abs_diff(a: u8, b: u8) -> u8 {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

fn read_image(path: &String) -> Image {
    if let Ok(img) = image::open(path) {
        let (width, height) = img.dimensions();
        let mut pixels: Vec<Vec<Rgba<u8>>> = vec![vec![Rgba([0, 0, 0, 0]); width as usize]; height as usize];
        for y in 0..height {
            for x in 0..width {
                let pixel_color = img.get_pixel(x, y);
                pixels[y as usize][x as usize] = pixel_color;
            }
        }
        return Image {
            width: width,
            height: height,
            pixels: pixels,
        };
    } else {
        panic!("Failed to open the image '{}'.", path);
    }
}

fn store_svg_as_png(filepath: String, svg_data: &String, width: u32, height: u32) {
    let opt = usvg::Options::default();
    let db = fontdb::Database::new();

    let tree = usvg::Tree::from_str(&svg_data, &opt, &db).unwrap();
    let transformation = tiny_skia_path::Transform::identity();
    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
    resvg::render(&tree, transformation, &mut pixmap.as_mut());
    pixmap.save_png(filepath).unwrap();
}
