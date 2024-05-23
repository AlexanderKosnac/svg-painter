use std::cmp;
use rand::Rng;

use crate::genetic::color::Rgba;

use crate::genetic::{Base, Genome};

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
        Self {
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
        let m = 5;
        let mut rng = rand::thread_rng();
        self.x = self.x + rng.gen_range(-m..m);
        self.y = self.y + rng.gen_range(-m..m);
        self.r = cmp::max(self.r + rng.gen_range(-m..m), 1);
        self.color.mutate(rng.gen_range(0.0..20.0));
    }
}

impl Clone for CircleBase {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            r: self.r,
            color: self.color.clone(),
            max_r: self.max_r,
        }
    }
}

pub struct TriangleBase {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    color: Rgba,
}

impl Base for TriangleBase {
    fn new(max_x: u32, max_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x1: rng.gen_range(0..max_x) as i32,
            y1: rng.gen_range(0..max_y) as i32,
            x2: rng.gen_range(0..max_x) as i32,
            y2: rng.gen_range(0..max_y) as i32,
            x3: rng.gen_range(0..max_x) as i32,
            y3: rng.gen_range(0..max_y) as i32,
            color: Rgba::new_rand(),
        }
    }

    fn express(&self) -> String {
        let point_seq = format!("{},{} {},{} {},{}", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3);
        return format!("<polygon points=\"{}\" fill-opacity=\"{:.3}\" fill=\"{}\"/>", point_seq, (self.color.a as f64)/255.0, self.color.as_hex());
        
    }

    fn mutate(&mut self) {
        let m = 5;
        let mut rng = rand::thread_rng();
        self.x1 = self.x1 + rng.gen_range(-m..m);
        self.y1 = self.y1 + rng.gen_range(-m..m);
        self.x2 = self.x2 + rng.gen_range(-m..m);
        self.y2 = self.y2 + rng.gen_range(-m..m);
        self.x3 = self.x3 + rng.gen_range(-m..m);
        self.y3 = self.y3 + rng.gen_range(-m..m);
        self.color.mutate(rng.gen_range(0.0..20.0));
    }
}

impl Clone for TriangleBase {
    fn clone(&self) -> Self {
        Self {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            x3: self.x3,
            y3: self.y3,
            color: self.color.clone(),
        }
    }
}

static STROKES: [&str; 4] = [
];

pub struct StrokeBase {
    stroke_idx: usize,
    x: i32,
    y: i32,
    rotation: i32,
    scale_x: f32,
    scale_y: f32,
    color: Rgba,
}

impl Base for StrokeBase {
    fn new(max_x: u32, max_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        let scale = rng.gen_range(0.5..2.0);
        Self {
            stroke_idx: rng.gen_range(0..STROKES.len()) as usize,
            x: rng.gen_range(0..max_x) as i32,
            y: rng.gen_range(0..max_y) as i32,
            rotation: rng.gen_range(0..360) as i32,
            scale_x: scale,
            scale_y: scale,
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
        //self.stroke_idx = (self.stroke_idx + rng.gen_range(0..1)) % 3;
        self.x = self.x + rng.gen_range(-m..m);
        self.y = self.y + rng.gen_range(-m..m);
        self.rotation = (self.rotation + rng.gen_range(-m..m)) % 360;
        self.scale_x = self.scale_x + rng.gen_range(-0.3..0.3);
        self.scale_y = self.scale_y + rng.gen_range(-0.3..0.3);
        self.color.mutate(rng.gen_range(0.0..20.0));
    }
}

impl Clone for StrokeBase {
    fn clone(&self) -> Self {
        Self {
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

pub struct SvgElementGenome<T: Base> {
    sequence: Vec<T>,
    width: u32,
    height: u32,
}

impl<T: Base> Genome for SvgElementGenome<T> {
    fn new(genome_size: u32, width: u32, height: u32) -> Self {
        Self {
            sequence: (0..genome_size).map(|_| T::new(width, height)).collect(),
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
        let idx = rng.gen_range(0..self.sequence.len()) as usize;
        let mut candidate = &mut self.sequence[idx];
        candidate.mutate();
    }

    fn insertion(&mut self) {
        self.sequence.push(T::new(self.width, self.height));
    }

    fn len(&self) -> usize {
        return self.sequence.len();
    }
}

impl<T: Base + Clone> Clone for SvgElementGenome<T> {
    fn clone(&self) -> Self {
        Self {
            sequence: self.sequence.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

unsafe impl<T: Base> Send for SvgElementGenome<T> {}
