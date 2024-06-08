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

use crate::genetic::color::Rgba;
use crate::Controller;

pub mod color;

pub struct Experiment {
    controller_rc: Rc<RefCell<Controller>>,
    target: tiny_skia::Pixmap,
    population: Vec<(SvgElementGenome, f64)>,
    generation: u64,
    fitness_history: Vec<f64>,
}

impl Experiment {

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
        println!("Generation {:5}; avg. fit.: {}", self.generation, last_avg_fitness);
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

static STROKES: [&str; 3] = [
    "<rect id=\"stroke-0\" width=\"100\" height=\"25\"/>",
    "<rect id=\"stroke-1\" width=\"100\" height=\"50\"/>",
    "<rect id=\"stroke-2\" width=\"100\" height=\"75\"/>",
];

pub struct StrokeBase {
    controller_rc: Rc<RefCell<Controller>>,
    stroke_idx: usize,
    x: i32,
    y: i32,
    rotation: i32,
    scale_x: f32,
    scale_y: f32,
    color: Rgba,
}

impl StrokeBase {
    pub fn new(controller_rc: Rc<RefCell<Controller>>) -> Self {
        let mut rng = rand::thread_rng();
        let (x, y) = controller_rc.borrow().get_xy();
        let (scale_x, scale_y) = controller_rc.borrow().get_scale();
        Self {
            controller_rc: controller_rc,
            stroke_idx: rng.gen_range(0..STROKES.len()) as usize,
            x: x,
            y: y,
            rotation: rng.gen_range(0..360),
            scale_x: scale_x,
            scale_y: scale_y,
            color: Rgba::new_rand(),
        }
    }

    fn express(&self) -> String {
        let stroke = format!("<use href=\"#stroke-{}\"/>", self.stroke_idx);
        let transformations = format!("translate({} {}) rotate({}) scale({:.5} {:.5})", self.x, self.y, self.rotation, self.scale_x, self.scale_y);
        return format!("<g fill-opacity=\"{:.3}\" fill=\"{}\" transform=\"{}\">{}</g>", (self.color.a as f64)/255.0, self.color.as_hex(), transformations, stroke);
    }

    fn mutate(&mut self) {
        let m = 5;
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=2) {
            0 => {
                self.x += rng.gen_range(-m..m);
                self.y += rng.gen_range(-m..m);
            },
            1 => self.rotation = (self.rotation + rng.gen_range(-m..m)) % 360,
            2 => self.color.mutate(rng.gen_range(0.0..20.0)),
            _ => panic!("Cannot happen."),
        }
    }
}

impl Clone for StrokeBase {
    fn clone(&self) -> Self {
        Self {
            controller_rc: Rc::clone(&self.controller_rc),
            stroke_idx: self.stroke_idx,
            x: self.x,
            y: self.y,
            rotation: self.rotation,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            color: self.color.clone(),
        }
    }
}

pub struct SvgElementGenome {
    controller_rc: Rc<RefCell<Controller>>,
    sequence: Vec<StrokeBase>,
    sequence_fixed: Vec<StrokeBase>,
    width: u32,
    height: u32,
}

impl SvgElementGenome {
    fn new(controller_rc: Rc<RefCell<Controller>>, width: u32, height: u32) -> Self {
        Self {
            controller_rc: controller_rc,
            sequence: Vec::new(),
            sequence_fixed: Vec::new(),
            width: width,
            height: height,
        }
    }

    fn express(&self) -> String {
        let expressed: String = self.sequence_fixed.iter().chain(self.sequence.iter()).map(|b| b.express()).collect::<Vec<String>>().join("\n");
        return format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n<def>\n{}\n</def>\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n{expressed}\n</svg>",
            self.width, self.height, STROKES.join("\n")
        );
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        let range_max = self.sequence.len();
        let bases_to_mutate = cmp::max(1, (range_max as f64 * 0.05) as u64);
        let candidates: Vec<usize> = (0..bases_to_mutate).map(|_| rng.gen_range(0..range_max)).collect();

        for c in candidates {
            let candidate = &mut self.sequence[c];
            candidate.mutate();
        }
    }

    fn cross_with(&self, other: &SvgElementGenome) -> SvgElementGenome {
        let crossover_points = util::random_points_in_range(3, 0, self.sequence.len() as u64);

        let mut new_sequence = Vec::new();
        let mut from_self = true;
        let mut slice_start = 0;
        for i in &crossover_points[1..] {
            let source = if from_self { &self.sequence } else { &other.sequence };
            from_self = !from_self;
            new_sequence.extend_from_slice(&source[slice_start..(*i as usize)]);
            slice_start = *i as usize;
        }
        SvgElementGenome {
            controller_rc: Rc::clone(&self.controller_rc),
            sequence: new_sequence,
            sequence_fixed: self.sequence_fixed.clone(),
            width: self.width,
            height: self.height,
        }
    }

    fn insertion(&mut self, n: u64) {
        for _ in 0..n {
            self.sequence.push(StrokeBase::new(Rc::clone(&self.controller_rc)));
        }
    }

    fn fixate(&mut self) {
        self.sequence.append(&mut self.sequence_fixed);
    }
}

impl Clone for SvgElementGenome {
    fn clone(&self) -> Self {
        Self {
            controller_rc: Rc::clone(&self.controller_rc),
            sequence: self.sequence.clone(),
            sequence_fixed: self.sequence_fixed.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

unsafe impl Send for SvgElementGenome {}
