use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use image::{GenericImageView};
use tiny_skia::{PremultipliedColorU8, ColorU8};

use resvg;
use usvg;
use fontdb;
use tiny_skia;
use tiny_skia_path;

pub mod genetic;
use genetic::{Genome, CircleGenome};

pub mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    let raster_image_path = &args[1];
    let n_generations = 10000000;

    let genome_size = 500;
    let population_size = 100;

    evolve::<CircleGenome>(raster_image_path, n_generations, genome_size, population_size);
}

fn evolve<T: Genome + Clone>(raster_image_path: &String, n_generations: u64, genome_size: u32, population_size: u64) {
    let target = read_image(raster_image_path);

    let mut population: Vec<(T, u64)> = (0..population_size).map(|_| (T::new(genome_size, target.width(), target.height()), 0)).collect();

    let mut generation: u64 = 0;
    loop {
        generation += 1;
        println!("Generation {}; {} individuals", generation, population.len());
        let dir = format!("build/generation_{:0>3}", generation);
        fs::create_dir_all(&dir).expect("Unable to create generation directory");

        let mut i = 0;
        for individual in &mut population {
            let svg_data = individual.0.express();
            let mut candidate = tiny_skia::Pixmap::new(target.width(), target.height()).unwrap();
            render_svg_into_pixmap(&svg_data, &mut candidate);
            individual.1 = pixmap_distance(&candidate, &target);
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

        population = setup_population::<T>(fittest, population_size);
    }
}

fn setup_population<T: Genome + Clone>(base_individuals: &[(T, u64)], population_size: u64) -> Vec<(T, u64)> {
    let mut population: Vec<(T, u64)> = Vec::new();

    let individuals_per_genome = population_size / (base_individuals.len() as u64);
    for individual in base_individuals {
        for _ in 0..individuals_per_genome {
            let mut new_genome = individual.0.clone();
            new_genome.mutate();
            population.push((new_genome, 0));
        }
    }

    return population;
}

fn pixmap_distance(d1: &tiny_skia::Pixmap, d2: &tiny_skia::Pixmap) -> u64 {
    if d1.width() != d2.width() || d1.height() != d2.height() {
        panic!("Pixmaps of different dimensions can not be compared. Got {}x{} and {}x{}.", d1.width(), d1.height(), d2.width(), d2.height());
    }
    let pixels1 = d1.pixels();
    let pixels2 = d2.pixels();
    return (0..pixels1.len()).map(|i| coloru8_dot_product(&pixels1[i], &pixels2[i]) as u64).sum::<u64>();
}

fn coloru8_dot_product(c1: &tiny_skia::PremultipliedColorU8, c2: &tiny_skia::PremultipliedColorU8) -> u32 {
    return
        (c1.red() as u32 * c2.red() as u32) +
        (c1.green() as u32 * c2.green() as u32) +
        (c1.blue() as u32 * c2.blue() as u32) +
        (c1.alpha() as u32 * c2.alpha() as u32);
}

fn read_image(path: &String) -> tiny_skia::Pixmap {
    println!("-- {}", path);
    let path_obj = Path::new(path);
    println!("++ {}", path_obj.display());
    return tiny_skia::Pixmap::load_png(&path_obj).expect("Failed to open image");
}

fn render_svg_into_pixmap(svg_data: &String, pixmap: &mut tiny_skia::Pixmap) {
    let opt = usvg::Options::default();
    let db = fontdb::Database::new();

    let tree = usvg::Tree::from_str(&svg_data, &opt, &db).unwrap();
    let transformation = tiny_skia_path::Transform::identity();
    resvg::render(&tree, transformation, &mut pixmap.as_mut());
}
