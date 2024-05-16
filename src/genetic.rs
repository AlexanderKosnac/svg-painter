pub mod color;

use std::cmp;
use rand::Rng;

use delaunator::{Point, triangulate};

use crate::genetic::color::Rgba;
use crate::util;

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
        let max_r = (max_x+max_y)/2/2;
        CircleBase {
            x: rng.gen_range(0..max_x) as i32,
            y: rng.gen_range(0..max_y) as i32,
            r: rng.gen_range(1..max_r) as i32,
            color: Rgba::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)),
            max_r: max_r,
        }
    }

    fn express(&self) -> String {
        return format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill-opacity=\"{:.3}\" fill=\"{}\"/>", self.x, self.y, self.r, (self.color.a as f64)/255.0, self.color.as_hex());
    }

    fn mutate(&mut self) {
        let strength = 100;
        let mut rng = rand::thread_rng();
        self.x = self.x + rng.gen_range(-strength..strength);
        self.y = self.y + rng.gen_range(-strength..strength);
        self.r = util::bound(self.r + rng.gen_range(-strength..strength), 1, self.max_r as i32);
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

pub struct PointBase {
    x: i32,
    y: i32,
    color: Rgba,
}

impl Base for PointBase {
    fn new(max_x: u32, max_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        PointBase {
            x: rng.gen_range(0..max_x) as i32,
            y: rng.gen_range(0..max_y) as i32,
            color: Rgba::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)),
        }
    }

    fn express(&self) -> String {
        return format!("");
    }

    fn mutate(&mut self) {
        let m = 100;
        let mut rng = rand::thread_rng();
        self.x = self.x + rng.gen_range(-m..m);
        self.y = self.y + rng.gen_range(-m..m);
        self.color.mutate(rng.gen_range(0.0..10.0));
    }
}

impl Clone for PointBase {
    fn clone(&self) -> Self {
        PointBase {
            x: self.x,
            y: self.y,
            color: self.color.clone(),
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
        return format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n{expressed_string}\n</svg>", self.width, self.height);
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_sequence = Vec::new();
        for base in &self.sequence {
            let throw = rng.gen::<f64>();
            let mut new_base = base.clone();
            if throw > 0.8 {
                new_base.mutate();
            }
            if throw < 0.995 {
                new_sequence.push(new_base);
            }
            if throw > 0.99 {
                let mut insertion = base.clone();
                insertion.mutate();
                new_sequence.push(insertion);
            }
        }
        self.sequence = new_sequence;
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

pub struct DelaunayGenome {
    sequence: Vec<PointBase>,
    width: u32,
    height: u32,
}

impl Genome for DelaunayGenome {
    fn new(genome_size: u32, width: u32, height: u32) -> Self {
        DelaunayGenome {
            sequence: (0..genome_size).map(|_| PointBase::new(width, height)).collect(),
            width: width,
            height: height,
        }
    }

    fn express(&self) -> String {
        let mut points: Vec<delaunator::Point> = self.sequence.iter().map(|it| Point { x: it.x as f64, y: it.y as f64 }).collect();
        points.push(Point { x: 0.0, y: 0.0 });
        points.push(Point { x: self.width as f64, y: 0.0 });
        points.push(Point { x: 0.0, y: self.height as f64 });
        points.push(Point { x: self.width as f64, y: self.height as f64 });

        let triangulation = triangulate(&points);
        let triangles = triangulation.triangles;

        let mut expressed: Vec<String> = Vec::new();
        for n in 0..=(triangles.len()/3-3) {
            let base = n*3;
            let t1 = &points[triangles[base+0]];
            let t2 = &points[triangles[base+1]];
            let t3 = &points[triangles[base+2]];
            let svg_point_format = format!("{},{} {},{} {},{}", t1.x, t1.y, t2.x, t2.y, t3.x, t3.y);
            let polygon = format!("<polygon points=\"{}\" fill=\"black\" stroke=\"white\" stroke-width=\"1\"/>", svg_point_format);
            expressed.push(polygon);
        }

        let expressed_string: String = expressed.join("\n");
        return format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n{expressed_string}\n</svg>", self.width, self.height);
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_sequence = Vec::new();
        for base in &self.sequence {
            let throw = rng.gen::<f64>();
            let mut new_base = base.clone();
            if throw > 0.9 {
                new_base.mutate();
            }
            if throw < 0.995 {
                 new_sequence.push(new_base);
            }
            if throw > 0.99 {
                let mut insertion = base.clone();
                insertion.mutate();
                new_sequence.push(insertion);
            }
        }
        self.sequence = new_sequence;
    }

    fn len(&self) -> usize {
        return self.sequence.len();
    }
}

impl Clone for DelaunayGenome {
    fn clone(&self) -> Self {
        DelaunayGenome {
            sequence: self.sequence.clone(),
            width: self.width,
            height: self.height,
        }
    }
}
