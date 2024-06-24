use tiny_skia;
use tiny_skia::PixmapPaint;
use tiny_skia::Transform;

use std::fs::File;
use std::io::Write;

use rand::Rng;

use crate::util;

use crate::Controller;
use crate::genetic::color::Rgba;
use crate::genetic::base::StrokeBase;

pub mod color;
pub mod base;

pub enum FileType {
    SVG,
    PNG,
}

pub struct ImageApproximation {
    target: tiny_skia::Pixmap,
    strokes: Vec<StrokeBase>,
    pixmap_render: tiny_skia::Pixmap,
    fitness: f64,
}

impl ImageApproximation {

    pub fn new(target_image_path: &String) -> Self {
        let target = util::read_image(target_image_path);
        let (width, height) = (target.width(), target.height());
        Self {
            target: target,
            strokes: Vec::new(),
            pixmap_render: tiny_skia::Pixmap::new(width, height).unwrap(),
            fitness: f64::MAX,
        }
    }

    pub fn write_to_file(&self, filetype: &FileType, path: &String) {
        let expression = self.express();
        match filetype {
            FileType::SVG => {
                let mut f = File::create(path).expect("Unable to create SVG file");
                f.write_all(expression.as_bytes()).expect("Unable to write data to disk");
            },
            FileType::PNG => {
                let mut canvas = tiny_skia::Pixmap::new(self.target.width(), self.target.height()).unwrap();
                util::render_svg_into_pixmap(&expression, &mut canvas);
                canvas.save_png(path).expect("Unable to create PNG file");
            },
        }
    }

    pub fn add_stroke(&mut self, controller: &Controller) -> bool {
        let mut rng = rand::thread_rng();

        let mut top_stroke = StrokeBase::new();
        top_stroke.set_xy(controller.get_xy());
        top_stroke.set_rotation(rng.gen_range(0..360));
        top_stroke.set_scale(controller.get_scale());

        match self.approximate_average_color_in_stroke(&top_stroke) {
            Some(c) => top_stroke.set_color(c),
            None => return false,
        };

        let mut top_render = self.get_render_with_stroke(&top_stroke);
        let mut top_fitness = util::pixmap_distance(&top_render, &self.target);

        let mut attempts = 0;
        while attempts < controller.get_max_attempts() {
            let mut new_stroke = top_stroke.clone();
            new_stroke.mutate(controller);

            match self.approximate_average_color_in_stroke(&top_stroke) {
                Some(c) => top_stroke.set_color(c),
                None => {
                    attempts += 1;
                    continue;
                },
            };

            let new_render = self.get_render_with_stroke(&new_stroke);
            let new_fitness = util::pixmap_distance(&new_render, &self.target);

            if new_fitness < top_fitness {
                top_stroke = new_stroke;
                top_render = new_render;
                top_fitness = new_fitness;
                attempts = 0;
            } else {
                attempts += 1;
            }
        }
        return if top_fitness < self.fitness {
            self.strokes.push(top_stroke);
            self.pixmap_render = top_render;
            self.fitness = top_fitness;
            true
        } else {
            false
        };
    }

    pub fn express(&self) -> String {
        let expressed: String = self.strokes.iter().map(|b| b.express()).collect::<Vec<String>>().join("\n");
        let (width, height) = (self.target.width(), self.target.height());
        let defs = base::STROKES.join("\n");
        format!("<svg width=\"{width}\" height=\"{height}\" xmlns=\"http://www.w3.org/2000/svg\">\n<def>\n{defs}\n</def>\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n{expressed}\n</svg>")
    }

    pub fn express_stroke(&self, stroke: &StrokeBase) -> String {
        let expressed = stroke.express();
        let (width, height) = (self.target.width(), self.target.height());
        let defs = base::STROKES.join("\n");
        format!("<svg width=\"{width}\" height=\"{height}\" xmlns=\"http://www.w3.org/2000/svg\">\n<def>\n{defs}\n</def>\n{expressed}\n</svg>")
    }

    pub fn get_render_with_stroke(&mut self, stroke: &StrokeBase) -> tiny_skia::Pixmap {
        let mut stroke_render = tiny_skia::Pixmap::new(self.target.width(), self.target.height()).unwrap();
        util::render_svg_into_pixmap(&self.express_stroke(&stroke), &mut stroke_render);

        let mut render = self.pixmap_render.clone();
        render.draw_pixmap(0, 0, stroke_render.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        return render;
    }

    pub fn average_color_in_stroke(&self, stroke: &StrokeBase) -> Rgba {
        let mut mask = tiny_skia::Pixmap::new(self.target.width(), self.target.height()).unwrap();
        util::render_svg_into_pixmap(&self.express_stroke(&stroke), &mut mask);

        let target_pixels = self.target.pixels();

        let mut colors = (0, 0, 0);
        let mut c = 0;
        let mut idx = 0;
        for pixel in mask.pixels() {
            if pixel.alpha() > 0 {
                let target_pixel = target_pixels[idx];
                colors.0 += target_pixel.red() as u64;
                colors.1 += target_pixel.green() as u64;
                colors.2 += target_pixel.blue() as u64;
                c += 1;
            }
            idx += 1;
        }
        if c == 0 {
            Rgba::new(0, 0, 0, 255)
        } else {
            Rgba::new((colors.0/c) as u8, (colors.1/c) as u8, (colors.2/c) as u8, 255)
        }
    }

    pub fn approximate_average_color_in_stroke(&self, stroke: &StrokeBase) -> Option<Rgba> {
        let (target_width, target_height) = (self.target.width() as i32, self.target.height() as i32);

        let mut colors = (0, 0, 0);
        let mut c = 0;
        for xy in stroke.approximate_pixels() {
            if xy.0 < 0 || xy.1 < 0 || xy.0 >= target_width || xy.1 >= target_height {
                continue;
            }
            let target_pixel = self.target.pixel(xy.0 as u32, xy.1 as u32).expect("Could not get pixel. Is checked, should be impossible.");
            colors.0 += target_pixel.red() as u64;
            colors.1 += target_pixel.green() as u64;
            colors.2 += target_pixel.blue() as u64;
            c += 1;
        }

        if c > 0 { Some(Rgba::new((colors.0/c) as u8, (colors.1/c) as u8, (colors.2/c) as u8, 255)) } else { None }
    }

    pub fn target_approximation_diffmap(&self) -> tiny_skia::Pixmap {
        util::image::abs_diff_graylevel_heatmap(&self.target, &self.pixmap_render)
    }
}
