use std::env;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

pub mod genetic;
use genetic::*;

pub mod util;

static BUILD: &str = "build";

fn main() {
    let args: Vec<String> = env::args().collect();

    let raster_image_path = &args[1];

    let _genome_size = 0;
    let population_size = 50;

    fs::create_dir_all(String::from(BUILD)).expect("Unable to create build directory");
    fs::copy(raster_image_path, format!("{BUILD}/trgt.png")).expect("Could not copy target file");

    let sobel_result = util::image::sobel(&util::read_image(raster_image_path));
    sobel_result.save_png(String::from("build/sobel.png")).expect("Unable to create Sobel file");
    let controller_rc = Rc::new(RefCell::new(Controller::new(&all_white)));
    let mut env = Experiment::new(Rc::clone(&controller_rc), raster_image_path, population_size);

    controller_rc.borrow_mut().set_scale(0.9, 0.9);
    env.insertion_on_all_individuals(10);

    let mut stage = 1;
    loop {
        println!("Stage {stage}");
        env.evolve();
        env.fixate_all_individuals();

        if stage % 10 == 0 {
            let scale = controller_rc.borrow().get_scale();
            let factor = 0.5;
            controller_rc.borrow_mut().set_scale(scale.0 * factor, scale.1 * factor);
        }

        env.insertion_on_all_individuals(1);
        stage += 1;
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

    pub fn build_base(&mut self) -> StrokeBase {
        let xy = self.mask.sample_random_xy();
        StrokeBase::new(xy.0 as i32, xy.1 as i32, self.scale_x, self.scale_y)
    }
}