use std::env;
use std::fs;

pub mod genetic;
use genetic::*;

pub mod util;

static BUILD: &str = "build";

fn main() {
    let args: Vec<String> = env::args().collect();

    let raster_image_path = &args[1];

    fs::create_dir_all(String::from(BUILD)).expect("Unable to create build directory");
    fs::copy(raster_image_path, format!("{BUILD}/trgt.png")).expect("Could not copy target file");

    let target = util::read_image(raster_image_path);

    //let gauss_result = util::image::gaussian_blur_from_gaussian_function(&target, 2.0, 3);
    let sobel_result = util::image::sobel(&target);

    let mask = sobel_result;
    mask.save_png(format!("{BUILD}/mask.png")).expect("Unable to create Mask file");

    let mut all_white = tiny_skia::Pixmap::new(target.width(), target.height()).unwrap();
    all_white.fill(tiny_skia::Color::WHITE);

    let mut controller = Controller::new(&mask);
    controller.set_scale(calc_scale(&target, 1));

    let mut approx = ImageApproximation::new(raster_image_path);
    approx.write_to_file(&genetic::FileType::SVG, &format!("{BUILD}/expr.svg"));

    let mut stage = 1;
    loop {
        let success = approx.add_stroke(&controller);
        if !success {
            stage += 1;
            let scale = calc_scale(&target, stage);
            controller.set_scale(scale);
        }

        approx.write_to_file(&genetic::FileType::SVG, &format!("{BUILD}/expr.svg"));
        approx.write_to_file(&genetic::FileType::PNG, &format!("{BUILD}/expr.png"));
        approx.target_approximation_diffmap().save_png(format!("{BUILD}/diff.png")).expect("Unable to create diff file");
    }
}

pub struct Controller {
    mask: util::image::GraylevelMask,
    scale_x: f32,
    scale_y: f32,
}

impl Controller {

    pub fn new(src_image: &tiny_skia::Pixmap) -> Self {
        Self {
            mask: util::image::GraylevelMask::from(src_image),
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }

    pub fn get_xy(&self) -> (i32, i32) {
        let xy = self.mask.sample_random_xy();
        (xy.0 as i32, xy.1 as i32)
    }

    pub fn set_scale(&mut self, scale: (f32, f32)) {
        self.scale_x = scale.0;
        self.scale_y = scale.1;
    }

    pub fn get_scale(&self) -> (f32, f32) {
        (self.scale_x, self.scale_y)
    }
}

fn calc_scale(target: &tiny_skia::Pixmap, stage: u32) -> (f32, f32) {
    (
        (target.width() as f32)/(genetic::base::STROKE_DIMENSION.0 * 4.0 * stage as f32),
        (target.height() as f32)/(genetic::base::STROKE_DIMENSION.1 * 4.0 * stage as f32),
    )
}