pub mod color;

use std::cmp;
use rand::Rng;

use crate::genetic::color::Rgba;

pub struct Base {
    x: u32,
    y: u32,
    r: u32,
    color: Rgba,
}

impl Base {
    pub fn new(max_x: u32, max_y: u32) -> Base {
        let mut rng = rand::thread_rng();
        Base {
            x: rng.gen_range(0..max_x),
            y: rng.gen_range(0..max_y),
            r: rng.gen_range(1..max_x/20),
            color: Rgba::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)),
        }
    }

    pub fn express(&self) -> String {
        return format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill-opacity=\"{:.3}\" fill=\"{}\"/>", self.x, self.y, self.r, (self.color.a as f64)/255.0, self.color.as_hex());
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = self.x + rng.gen_range(0..10);
        self.y = self.y + rng.gen_range(0..10);
        self.r = cmp::min(self.r + rng.gen_range(0..10), 20);
        self.color.mutate(rng.gen_range(0.0..10.0));
    }
}

impl Clone for Base {
    fn clone(&self) -> Self {
        Base {
            x: self.x,
            y: self.y,
            r: self.r,
            color: self.color.clone(),
        }
    }
}

pub struct Genome {
    sequence: Vec<Base>,
    width: u32,
    height: u32,
}

impl Genome {
    pub fn new(genome_size: u32, width: u32, height: u32) -> Genome {
        Genome {
            sequence: (0..genome_size).map(|_| Base::new(width, height)).collect(),
            width: width,
            height: height,
        }
    }

    pub fn express(&self) -> String {
        let expressed: Vec<String> = self.sequence.iter().map(|b| b.express()).collect();
        let expressed_string: String = expressed.join("\n");
        return format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n{expressed_string}\n</svg>", self.width, self.height);
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_sequence = Vec::new();
        for base in &self.sequence {
            let throw = rng.gen::<f64>();
            let mut new_base = base.clone();
            if throw > 0.9 {
                new_base.mutate();
            }
            new_sequence.push(new_base);
            if throw > 0.99 {
                let mut insertion = base.clone();
                insertion.mutate();
                new_sequence.push(insertion);
            }
        }
        self.sequence = new_sequence;
    }

    pub fn len(&self) -> usize {
        return self.sequence.len();
    }
}

impl Clone for Genome {
    fn clone(&self) -> Self {
        Genome {
            sequence: self.sequence.clone(),
            width: self.width,
            height: self.height,
        }
    }
}
