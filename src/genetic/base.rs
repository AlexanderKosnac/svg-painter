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
use crate::genetic::genome::SvgElementGenome;

pub static STROKES: [&str; 4] = [
    "<rect id=\"stroke-0\" width=\"100\" height=\"25\"/>",
    "<rect id=\"stroke-1\" width=\"100\" height=\"50\"/>",
    "<rect id=\"stroke-2\" width=\"100\" height=\"75\"/>",
    "<rect id=\"stroke-3\" width=\"100\" height=\"100\"/>",
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

    pub fn express(&self) -> String {
        let stroke = format!("<use href=\"#stroke-{}\"/>", self.stroke_idx);
        let transformations = format!("translate({} {}) rotate({}) scale({:.5} {:.5})", self.x, self.y, self.rotation, self.scale_x, self.scale_y);
        return format!("<g fill-opacity=\"{:.3}\" fill=\"{}\" transform=\"{}\">{}</g>", (self.color.a as f64)/255.0, self.color.as_hex(), transformations, stroke);
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=2) {
            0 => (self.x, self.y) = self.controller_rc.borrow().get_xy(),
            1 => self.rotation = rng.gen_range(0..360),
            2 => self.color = Rgba::new_rand(),
            _ => panic!("Should be impossible. Check if range of random number properly matches the available options."),
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
