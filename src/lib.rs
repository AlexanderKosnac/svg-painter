use std::fs;

pub mod genetic;
use genetic::*;

pub mod util;

pub fn run(output_path: &String, raster_image_path: &String) {
    fs::create_dir_all(output_path).expect("Unable to create build directory");
    fs::copy(raster_image_path, format!("{output_path}/trgt.png")).expect("Could not copy target file");

    let target = util::read_image(raster_image_path);

    let mut mask = tiny_skia::Pixmap::new(target.width(), target.height()).unwrap();
    mask.fill(tiny_skia::Color::WHITE);

    let mut controller = Controller::new(&mask);
    controller.set_scale(calc_scale(&target, 1));

    let mut approx = ImageApproximation::new(raster_image_path);
    approx.write_to_file(&genetic::FileType::SVG, &format!("{output_path}/expr.svg"));

    let mut stage = 1;
    let mut failed_insertions = 0;
    loop {
        let success = approx.add_stroke(&controller);
        if success {
            failed_insertions = 0;

            approx.write_to_file(&genetic::FileType::SVG, &format!("{output_path}/expr.svg"));
            approx.write_to_file(&genetic::FileType::PNG, &format!("{output_path}/expr.png"));
    
            let new_mask = approx.target_approximation_diffmap();
            new_mask.save_png(format!("{output_path}/mask.png")).expect("Unable to create mask file");
            controller.set_mask_from_pixmap(&new_mask);
        } else {
            failed_insertions += 1;
            if failed_insertions == 10 {
                failed_insertions = 0;
                stage += 1;
                let scale = calc_scale(&target, stage);
                controller.set_scale(scale);
            }
        }
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

    pub fn set_mask_from_pixmap(&mut self, pixmap: &tiny_skia::Pixmap) {
        self.mask = util::image::GraylevelMask::from(pixmap);
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

    pub fn get_max_attempts(&self) -> u32 {
        25
    }
}

fn calc_scale(target: &tiny_skia::Pixmap, stage: u32) -> (f32, f32) {
    (
        (target.width() as f32)/(genetic::base::STROKE_DIMENSION.0 * 8.0 * stage as f32),
        (target.height() as f32)/(genetic::base::STROKE_DIMENSION.1 * 8.0 * stage as f32),
    )
}