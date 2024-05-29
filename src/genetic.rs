use tiny_skia;

use rayon::prelude::*;

use std::fs::File;
use std::io::Write;

use crate::Controller;

use crate::util;

use crate::BUILD;

pub mod color;
pub mod svg;

pub trait Base {
    fn new(max_x: u32, max_y: u32) -> Self;
    fn express(&self) -> String;
    fn mutate(&mut self);
}

pub trait Genome {
    fn new(genome_size: u32, width: u32, height: u32) -> Self;
    fn express(&self) -> String;
    fn mutate(&mut self);
    fn insertion(&mut self);
    fn len(&self) -> usize;
}

pub struct Experiment<T: Genome + Clone + Send> {
    controller: Controller,
    pub target: tiny_skia::Pixmap,
    pub population: Vec<(T, f64)>,
}

impl<T: Genome + Clone + Send> Experiment<T> {
    pub fn new(target_image_path: &String, population_size: u64, genome_size: u32) -> Self {
        let target = util::read_image(target_image_path);
        let dim = (target.width(), target.height());
        Self {
            controller: Controller::new(),
            target: target,
            population: (0..population_size).map(|_| (T::new(genome_size, dim.0, dim.1), 0.0)).collect(),
        }
    }

    pub fn evolve(&mut self) {
        loop {
            self.controller.on_new_generation();

            self.evaluate_population();
            self.sort_population_by_fitness();
            self.after_evaluation();

            if self.controller.stop_condition() {
                break;
            }

            self.repopulate();
        }
    }

    fn evaluate_population(&mut self) {
        self.population.par_iter_mut().for_each(|individual| {
            let mut candidate = tiny_skia::Pixmap::new(self.target.width(), self.target.height()).unwrap();
            util::render_svg_into_pixmap(&individual.0.express(), &mut candidate);
            individual.1 = util::pixmap_distance(&candidate, &self.target);
        });
    }

    fn after_evaluation(&self) {
        for individual in self.population.iter().take(1) {
            let base = format!("{BUILD}/expr");
            let expression = individual.0.express();

            let mut f = File::create(format!("{base}.svg")).expect("Unable to create SVG file");
            f.write_all(expression.as_bytes()).expect("Unable to write data");

            let mut candidate = tiny_skia::Pixmap::new(self.target.width(), self.target.height()).unwrap();
            util::render_svg_into_pixmap(&expression, &mut candidate);
            candidate.save_png(format!("{base}.png")).expect("Unable to create PNG file");
        }
    }

    fn sort_population_by_fitness(&mut self) {
        self.population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }

    fn repopulate(&mut self) {
        let fittest = &self.population[0..5];
        let individuals_per_genome = self.population.len() / fittest.len();

        let mut new_population = Vec::new();
        for individual in fittest {
            new_population.push((individual.0.clone(), 0.0));
            for _ in 0..(individuals_per_genome-1) {
                let mut new_genome = individual.0.clone();
                new_genome.mutate();
                new_population.push((new_genome, 0.0));
            }
        }
        self.population = new_population;
    }
}