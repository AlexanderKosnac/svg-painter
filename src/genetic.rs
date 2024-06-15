use tiny_skia;

use rayon::prelude::*;

use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

use rand_distr::Distribution;

use crate::util;

use crate::BUILD;

use std::cmp;
use rand::Rng;

use crate::Controller;
use crate::genetic::color::Rgba;
use crate::genetic::base::StrokeBase;
use crate::genetic::genome::SvgElementGenome;

pub mod color;
pub mod base;
pub mod genome;

pub struct ImageApproximation {
    controller_rc: Rc<RefCell<Controller>>,
    target: tiny_skia::Pixmap,
    population: Vec<(SvgElementGenome, f64)>,
    generation: u64,
    fitness_history: Vec<f64>,
}

impl ImageApproximation {

    pub fn new(controller_rc: Rc<RefCell<Controller>>, target_image_path: &String, population_size: u64) -> Self {
        let target = util::read_image(target_image_path);
        let dim = (target.width(), target.height());
        Self {
            controller_rc: Rc::clone(&controller_rc),
            target: target,
            population: (0..population_size).map(|_| (SvgElementGenome::new(Rc::clone(&controller_rc), dim.0, dim.1), 0.0)).collect(),
            generation: 0,
            fitness_history: Vec::new(),
        }
    }

    pub fn evolve(&mut self) {
        loop {
            self.on_new_generation();

            self.evaluate_population();
            self.sort_population_by_fitness();
            self.after_evaluation();

            if self.stop_condition() {
                break;
            }

            self.repopulate();
        }
    }

    pub fn fixate_all_individuals(&mut self) {
        for individual in &mut self.population {
            individual.0.fixate();
        }
    }

    pub fn insertion_on_all_individuals(&mut self, n: u64) {
        for individual in &mut self.population {
            individual.0.insertion(n);
            individual.1 = 0.0;
        }
    }

    fn on_new_generation(&mut self) {
        self.generation += 1;
        let last_avg_fitness = match self.fitness_history.last() {
            None => String::from("N/A"),
            Some(i) => format!("{:9.3}", i),
        };
        let mut fit_chg = 0.0;
        if self.generation > 2 {
            let idx = self.fitness_history.len();
            fit_chg = self.fitness_history[idx-2] - self.fitness_history[idx-1];
        }
        println!("Generation {:5}; avg. fit.: {}; fit. chg.: {:3.3}", self.generation, last_avg_fitness, fit_chg);
    }

    fn evaluate_population(&mut self) {
        self.population.par_iter_mut().for_each(|individual| {
            let mut candidate = tiny_skia::Pixmap::new(self.target.width(), self.target.height()).unwrap();
            util::render_svg_into_pixmap(&individual.0.express(), &mut candidate);
            individual.1 = util::pixmap_distance(&candidate, &self.target);
        });
    }

    fn after_evaluation(&mut self) {
        for individual in self.population.iter().take(1) {
            let base = format!("{BUILD}/expr");
            let expression = individual.0.express();

            let mut f = File::create(format!("{base}.svg")).expect("Unable to create SVG file");
            f.write_all(expression.as_bytes()).expect("Unable to write data");

            let mut candidate = tiny_skia::Pixmap::new(self.target.width(), self.target.height()).unwrap();
            util::render_svg_into_pixmap(&expression, &mut candidate);
            candidate.save_png(format!("{base}.png")).expect("Unable to create PNG file");
        }

        let fittest = &self.population[0..5];
        let avg_fitness = fittest.iter().map(|it| it.1).sum::<f64>()/fittest.len() as f64;
        self.fitness_history.push(avg_fitness);
    }

    fn sort_population_by_fitness(&mut self) {
        self.population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }

    fn stop_condition(&self) -> bool {
        let last_idx = self.fitness_history.len();
        let latest_fitness = self.fitness_history[last_idx-1];
        let window_size = 30;
        if last_idx < window_size {
            return false;
        }
        return (0..window_size).all(|i| (self.fitness_history[last_idx-i-2] - latest_fitness).abs() < 1.0);
    }

    fn repopulate(&mut self) {
        let fitness = self.population.iter().map(|p| p.1).collect::<Vec<f64>>();
        let max = fitness.iter().max_by(|a, b| a.total_cmp(b)).unwrap_or(&0.0);

        let fitness_inverted = fitness.iter().map(|f| max-f).collect::<Vec<f64>>();
        let sum = fitness_inverted.iter().sum::<f64>();

        let fitness_normalized = fitness_inverted.iter().map(|f| f/sum).collect::<Vec<f64>>();

        let dist = rand_distr::WeightedIndex::new(&fitness_normalized).unwrap();

        let mut rng = rand::thread_rng();
        let mut new_population = Vec::new();
        new_population.push((self.population[0].0.clone(), 0.0));
        for _ in 1..self.population.len() {
            let parent1 = &self.population[dist.sample(&mut rng)].0;
            let parent2 = &self.population[dist.sample(&mut rng)].0;
            let mut child = parent1.cross_with(&parent2);
            child.mutate();
            new_population.push((child, 0.0));
        }
        self.population = new_population;
    }
}
