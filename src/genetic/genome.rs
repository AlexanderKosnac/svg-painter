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
use crate::genetic::base::STROKES;
use crate::genetic::base::StrokeBase;

pub struct SvgElementGenome {
    controller_rc: Rc<RefCell<Controller>>,
    sequence: Vec<StrokeBase>,
    sequence_fixed: Vec<StrokeBase>,
    width: u32,
    height: u32,
}

impl SvgElementGenome {
    pub fn new(controller_rc: Rc<RefCell<Controller>>, width: u32, height: u32) -> Self {
        Self {
            controller_rc: controller_rc,
            sequence: Vec::new(),
            sequence_fixed: Vec::new(),
            width: width,
            height: height,
        }
    }

    pub fn express(&self) -> String {
        let expressed: String = self.sequence_fixed.iter().chain(self.sequence.iter()).map(|b| b.express()).collect::<Vec<String>>().join("\n");
        return format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n<def>\n{}\n</def>\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n{expressed}\n</svg>",
            self.width, self.height, STROKES.join("\n")
        );
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        let range_max = self.sequence.len();
        let bases_to_mutate = cmp::max(1, (range_max as f64 * 0.05) as u64);
        let candidates: Vec<usize> = (0..bases_to_mutate).map(|_| rng.gen_range(0..range_max)).collect();

        for c in candidates {
            self.sequence[c].mutate();
        }
    }

    pub fn cross_with(&self, other: &SvgElementGenome) -> SvgElementGenome {
        let mut rng = rand::thread_rng();

        let mut sequences = vec![&self.sequence, &other.sequence];
        sequences.sort_by(|a, b| a.len().cmp(&b.len()));
        let (short, long) = (sequences[0], sequences[1]);

        let crossover_point_idx = rng.gen_range(0..long.len());

        let mut new_sequence = Vec::new();
        new_sequence.extend_from_slice(&short[0..crossover_point_idx]);
        new_sequence.extend_from_slice(&long[crossover_point_idx..]);

        SvgElementGenome {
            controller_rc: Rc::clone(&self.controller_rc),
            sequence: new_sequence,
            sequence_fixed: self.sequence_fixed.clone(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn insertion(&mut self, n: u64) {
        for _ in 0..n {
            self.sequence.push(StrokeBase::new(Rc::clone(&self.controller_rc)));
        }
    }

    pub fn fixate(&mut self) {
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
