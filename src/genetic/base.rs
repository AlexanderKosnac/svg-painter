use rand::Rng;

use crate::Controller;
use crate::genetic::color::Rgba;

pub static STROKES: [&str; 4] = [
    "<rect id=\"stroke-0\" width=\"100\" height=\"100\"/>",
    "<rect id=\"stroke-1\" width=\"100\" height=\"75\"/>",
    "<rect id=\"stroke-2\" width=\"100\" height=\"50\"/>",
    "<rect id=\"stroke-3\" width=\"100\" height=\"25\"/>",
pub static STROKE_DIMENSION: (f32, f32) = (100.0, 100.0);
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

impl StrokeBase {

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            stroke_idx: rng.gen_range(0..STROKES.len()) as usize,
            x: 0,
            y: 0,
            rotation: 0,
            scale_x: 1.0,
            scale_y: 1.0,
            color: Rgba::new_black(),
        }
    }

    pub fn set_xy(&mut self, xy: (i32, i32)) {
        self.x = xy.0;
        self.y = xy.1;
    }

    pub fn set_scale(&mut self, scale: (f32, f32)) {
        self.scale_x = scale.0;
        self.scale_y = scale.1;
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        self.rotation = rotation;
    }

    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    pub fn express(&self) -> String {
        let stroke = format!("<use href=\"#stroke-{}\"/>", self.stroke_idx);
        let transformations = format!("translate({} {}) rotate({}) scale({:.5} {:.5})", self.x, self.y, self.rotation, self.scale_x, self.scale_y);
        let opacity = 0.9;
        let color = self.color.as_hex();
        format!("<g fill-opacity=\"{opacity:.3}\" fill=\"{color}\" transform=\"{transformations}\">{stroke}</g>")
    }

    pub fn mutate(&mut self, _controller: &Controller) {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=1) {
            0 => {
                self.x += rng.gen_range(-16..16);
                self.y += rng.gen_range(-16..16);
            },
            1 => {
                self.rotation += rng.gen_range(-90..90);
                self.rotation %= 360;
            },
            _ => panic!("Should be impossible. Check if range of random number properly matches the available options."),
        }
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
