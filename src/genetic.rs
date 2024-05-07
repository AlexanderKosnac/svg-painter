use std::cmp;
use rand::Rng;

use crate::util;

pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r: r, g: g, b: b, a: a }
    }

    pub fn as_hex(&self) -> String {
        return format!("#{:X}{:X}{:X}", self.r, self.g, self.b);
    }

    pub fn mutate(&mut self, magnitude: f64) {
        let mut rng = rand::thread_rng();
        let dir_vec: (f64, f64, f64) = (
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0)
        );
        let len = (dir_vec.0.powf(2.0) + dir_vec.1.powf(2.0) + dir_vec.2.powf(2.0)).sqrt();
        let normed = (dir_vec.0/len, dir_vec.1/len, dir_vec.2/len);
        let scaled = (normed.0 * magnitude, normed.1 * magnitude, normed.2 * magnitude);

        self.r = util::bounded_add(self.r, scaled.0 as i64);
        self.g = util::bounded_add(self.g, scaled.1 as i64);
        self.b = util::bounded_add(self.b, scaled.2 as i64);
        self.a += 0;
    }
}

impl Clone for Rgba {
    fn clone(&self) -> Self {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

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
        return format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" fill-opacity=\"0.5\"/>", self.x, self.y, self.r, self.color.as_hex());
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
        for base in &mut self.sequence {
            if rng.gen::<f64>() > 0.9 {
                base.mutate();
            }
        }
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
