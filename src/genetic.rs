pub mod color;

use std::cmp;
use rand::Rng;

use crate::genetic::color::Rgba;

pub trait Base {
    fn new(max_x: u32, max_y: u32) -> Self;
    fn express(&self) -> String;
    fn mutate(&mut self);
}

pub trait Genome {
    fn new(genome_size: u32, width: u32, height: u32) -> Self;
    fn express(&self) -> String;
    fn mutate(&mut self);
    fn len(&self) -> usize;
}

pub struct CircleBase {
    x: i32,
    y: i32,
    r: i32,
    color: Rgba,
    max_r: u32,
}

impl Base for CircleBase {
    fn new(max_x: u32, max_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        let max_r = (max_x+max_y)/2/16;
        CircleBase {
            x: rng.gen_range(0..max_x) as i32,
            y: rng.gen_range(0..max_y) as i32,
            r: rng.gen_range(1..max_r) as i32,
            color: Rgba::new_rand(),
            max_r: max_r,
        }
    }

    fn express(&self) -> String {
        return format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill-opacity=\"{:.3}\" fill=\"{}\"/>", self.x, self.y, self.r, (self.color.a as f64)/255.0, self.color.as_hex());
    }

    fn mutate(&mut self) {
        let m = 10;
        let mut rng = rand::thread_rng();
        self.x = self.x + rng.gen_range(-m..m);
        self.y = self.y + rng.gen_range(-m..m);
        self.r = cmp::max(self.r + rng.gen_range(-m..m), 1);
        self.color.mutate(rng.gen_range(0.0..10.0));
    }
}

impl Clone for CircleBase {
    fn clone(&self) -> Self {
        CircleBase {
            x: self.x,
            y: self.y,
            r: self.r,
            color: self.color.clone(),
            max_r: self.max_r,
        }
    }
}

pub struct CircleGenome {
    sequence: Vec<CircleBase>,
    width: u32,
    height: u32,
}

impl Genome for CircleGenome {
    fn new(genome_size: u32, width: u32, height: u32) -> Self {
        CircleGenome {
            sequence: (0..genome_size).map(|_| CircleBase::new(width, height)).collect(),
            width: width,
            height: height,
        }
    }

    fn express(&self) -> String {
        let expressed: Vec<String> = self.sequence.iter().map(|b| b.express()).collect();
        let expressed_string: String = expressed.join("\n");
        return format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n{expressed_string}\n</svg>", self.width, self.height);
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        for base in &mut self.sequence {
            if rng.gen::<f64>() > 0.95 {
                base.mutate();
            }
        }
    }

    fn len(&self) -> usize {
        return self.sequence.len();
    }
}

impl Clone for CircleGenome {
    fn clone(&self) -> Self {
        CircleGenome {
            sequence: self.sequence.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

unsafe impl Send for CircleGenome {}
