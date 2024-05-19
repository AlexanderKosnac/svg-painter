use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use rayon::prelude::*;

use resvg;
use usvg;
use fontdb;
use tiny_skia;
use tiny_skia_path;

pub mod genetic;
use genetic::Genome;
use genetic::svg::*;

pub mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    let raster_image_path = &args[1];
    let n_generations = 10000000;

    let genome_size = 1;
    let population_size = 50;

    evolve::<SvgElementGenome<CircleBase>>(raster_image_path, n_generations, genome_size, population_size);
}

fn evolve<T: Genome + Clone + Send>(raster_image_path: &String, n_generations: u64, genome_size: u32, population_size: u64) {
    let dir = String::from("build");
    fs::create_dir_all(&dir).expect("Unable to create build directory");

    fs::copy(raster_image_path, format!("{dir}/trgt.png")).expect("Could not copy target file");

    let target = read_image(raster_image_path);
    let dim = (target.width(), target.height());

    let min_fitness: u64 = dim.0 as u64 * dim.1 as u64 * 510; // 510 = sqrt((255-0)^2 + (255-0)^2 + (255-0)^2 + (255-0)^2)

    let mut population: Vec<(T, f64)> = (0..population_size).map(|_| (T::new(genome_size, dim.0, dim.1), 0.0)).collect();

    let mut history = vec![0.0; 30];
    let mut generation: u64 = 0;
    loop {
        generation += 1;
        println!("Generation {}; {} individuals; avg. f.: {:.2}", generation, population.len(), history[history.len()-1]);

        population.par_iter_mut().for_each(|individual| {
            let mut candidate = tiny_skia::Pixmap::new(dim.0, dim.1).unwrap();
            render_svg_into_pixmap(&individual.0.express(), &mut candidate);
            individual.1 = pixmap_distance(&candidate, &target);
        });

        population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let fittest = &population[0..5];
        let avg_fitness = fittest.iter().map(|it| it.1).sum::<f64>()/fittest.len() as f64;
        history.push(avg_fitness);

        if generation % 10 == 0 {
            for individual in population.iter().take(1) {
                println!("Individual: fitness {:.2}/{min_fitness}; Genome Size: {}", individual.1, individual.0.len());
                let base = format!("{}/expr", dir);
                let expression = individual.0.express();

                let mut f = File::create(format!("{base}.svg")).expect("Unable to create SVG file");
                f.write_all(expression.as_bytes()).expect("Unable to write data");

                let mut candidate = tiny_skia::Pixmap::new(dim.0, dim.1).unwrap();
                render_svg_into_pixmap(&expression, &mut candidate);
                candidate.save_png(format!("{base}.png")).expect("Unable to create PNG file");
            }
        }

        let last_idx = history.len();
        let fitness_converges = (0..30).all(|i| (history[last_idx-i-2] - avg_fitness).abs() < 5.0);

        if fitness_converges {
            println!("Convergence at {:.2}", avg_fitness);
        }

        if generation == n_generations {
            break;
        }

        population = setup_population::<T>(fittest, population_size, fitness_converges);
    }
}

fn setup_population<T: Genome + Clone + Send>(base_individuals: &[(T, f64)], population_size: u64, do_insertion: bool) -> Vec<(T, f64)> {
    let mut population: Vec<(T, f64)> = Vec::new();

    let individuals_per_genome = population_size / base_individuals.len() as u64;
    for individual in base_individuals {
        population.push((individual.0.clone(), 0.0));
        for _ in 0..(individuals_per_genome-1) {
            let mut new_genome = individual.0.clone();
            new_genome.mutate();
            if do_insertion {
                new_genome.insertion();
            }
            population.push((new_genome, 0.0));
        }
    }

    return population;
}

fn pixmap_distance(d1: &tiny_skia::Pixmap, d2: &tiny_skia::Pixmap) -> f64 {
    if d1.width() != d2.width() || d1.height() != d2.height() {
        panic!("Pixmaps of different dimensions can not be compared. Got {}x{} and {}x{}.", d1.width(), d1.height(), d2.width(), d2.height());
    }
    let pixels1 = d1.pixels();
    let pixels2 = d2.pixels();
    return (0..pixels1.len()).map(|i| coloru8_norm_2(&pixels1[i], &pixels2[i]) as f64).map(|i| i*i).sum::<f64>().sqrt();
}

fn coloru8_norm_2(c1: &tiny_skia::PremultipliedColorU8, c2: &tiny_skia::PremultipliedColorU8) -> f64 {
    let dr = c1.red() as i32 - c2.red() as i32;
    let dg = c1.green() as i32 - c2.green() as i32;
    let db = c1.blue() as i32 - c2.blue() as i32;
    let da = c1.alpha() as i32 - c2.alpha() as i32;
    return ((dr*dr + dg*dg + db*db + da*da) as f64).sqrt() as f64;
}

fn read_image(path: &String) -> tiny_skia::Pixmap {
    let path_obj = Path::new(path);
    return tiny_skia::Pixmap::load_png(&path_obj).expect("Failed to open image");
}

fn render_svg_into_pixmap(svg_data: &String, pixmap: &mut tiny_skia::Pixmap) {
    let opt = usvg::Options::default();
    let db = fontdb::Database::new();

    let tree = usvg::Tree::from_str(&svg_data, &opt, &db).unwrap();
    let transformation = tiny_skia_path::Transform::identity();
    resvg::render(&tree, transformation, &mut pixmap.as_mut());
}
