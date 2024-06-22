use rand_distr;
use rand_distr::Distribution;

use tiny_skia;

use crate::util;

static PI: f64 = 3.14159265359;
static EULER_E: f64 = 2.71828182846;

fn rgba_to_grayscale(c: &tiny_skia::PremultipliedColorU8) -> i32 {
    rgb_to_grayscale(&c.demultiply())
}

fn rgb_to_grayscale(c: &tiny_skia::ColorU8) -> i32 {
    (c.red() as f64 * 0.299 + c.green() as f64 * 0.587 + c.blue() as f64 * 0.114) as i32
}

fn get_canvas(input: &tiny_skia::Pixmap) -> (tiny_skia::Pixmap, i32, i32){
    let width = input.width() as i32;
    let height = input.height() as i32;
    let canvas = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
    (canvas, width, height)
}

pub fn sobel(input: &tiny_skia::Pixmap) -> tiny_skia::Pixmap {
    let def = tiny_skia::PremultipliedColorU8::from_rgba(0, 0, 0, 255).unwrap();

    let (mut canvas, width, height) = get_canvas(&input);
    let data = canvas.pixels_mut();

    let get_pixel = |x: i32, y: i32| -> i32 {
        let x_clamped = util::clamp(x, 0, width-1) as u32;
        let y_clamped = util::clamp(y, 0, height-1) as u32;
        let p = input.pixel(x_clamped, y_clamped).unwrap_or(def);
        rgb_to_grayscale(&p.demultiply())
    };

    for i in 0..width {
        for j in 0..height {
            let a = get_pixel(i-1, j-1); // a [ a d f ]
            let b = get_pixel(i-1, j  ); // b [ b . g ]
            let c = get_pixel(i-1, j+1); // c [ c e h ]
            let d = get_pixel(i  , j-1); // d
            let e = get_pixel(i  , j+1); // e
            let f = get_pixel(i+1, j-1); // f
            let g = get_pixel(i+1, j  ); // g
            let h = get_pixel(i+1, j+1); // h

            let gx = a + (2*b) + c - f + (-2*g) - h;
            let gy = a + (2*d) + f - c + (-2*e) - h;
            let mag = ((gx.pow(2) + gy.pow(2)) as f64).sqrt() as u8;

            let idx = (j * width + i) as usize;
            data[idx] = tiny_skia::PremultipliedColorU8::from_rgba(mag, mag, mag, 255).unwrap();
        }
    }

    return canvas;
}

pub fn gaussian_blur(input: &tiny_skia::Pixmap) -> tiny_skia::Pixmap {
    gaussian_blur_from_gaussian_function(input, 2.0, 3)
}

pub fn gaussian_blur_from_gaussian_function(input: &tiny_skia::Pixmap, sigma: f64, kernel_radius: u32) -> tiny_skia::Pixmap {
    let kernel = get_gaussian_blur_kernel(sigma, kernel_radius);
    gaussian_blur_with_kernel(input, &kernel)
}

pub fn gaussian_blur_with_kernel(input: &tiny_skia::Pixmap, kernel: &Vec<Vec<f64>>) -> tiny_skia::Pixmap {
    let def = tiny_skia::PremultipliedColorU8::from_rgba(0, 0, 0, 255).unwrap();

    let (mut canvas, width, height) = get_canvas(&input);
    let data = canvas.pixels_mut();

    let kernel_width = kernel[0].len() as i32;
    let kernel_height = kernel.len() as i32;
    let kernel_width_offset = (kernel_width as f64/2.0).floor() as i32;
    let kernel_height_offset = (kernel_height as f64/2.0).floor() as i32;

    let get_pixel = |x: i32, y: i32| -> [f64; 4] {
        let q = if x < 0 || y < 0 { def } else { input.pixel(x as u32, y as u32).unwrap_or(def) };
        let p = q.demultiply();
        [p.red() as f64, p.green() as f64, p.blue() as f64, p.alpha() as f64]
    };

    for i in 0..width {
        for j in 0..height {
            let mut c = [0.0, 0.0, 0.0, 0.0];
            for k in 0..kernel_width {
                for l in 0..kernel_height {
                    let p = get_pixel(i+k-kernel_width_offset, j+l-kernel_height_offset);
                    let idcs = (k as usize, l as usize);
                    c[0] += kernel[idcs.0][idcs.1] * p[0];
                    c[1] += kernel[idcs.0][idcs.1] * p[1];
                    c[2] += kernel[idcs.0][idcs.1] * p[2];
                    c[3] += kernel[idcs.0][idcs.1] * p[3];
                }
            }

            let idx = (j * width + i) as usize;
            data[idx] = tiny_skia::PremultipliedColorU8::from_rgba(c[0] as u8, c[1] as u8, c[2] as u8, c[3] as u8).unwrap();
        }
    }

    return canvas;
}

fn get_gaussian_blur_kernel(sigma: f64, matrix_radius: u32) -> Vec<Vec<f64>> {
    let sigma2 = sigma*sigma;
    let factor = 1.0/(2.0*PI*sigma2);
    let g = |x: i32, y: i32| -> f64 {
        factor * EULER_E.powf(-((x*x + y*y) as f64/(2.0*sigma2)))
    };
    let matrix_diameter = 1 + 2*matrix_radius as usize;
    let mut kernel = vec![vec![0.0; matrix_diameter]; matrix_diameter];

    let r = matrix_radius as i32;
    let mut sum = 0.0;
    for i in -r..=r {
        for j in -r..=r {
            let gauss = g(i, j);
            kernel[(i+r) as usize][(j+r) as usize] = gauss;
            sum += gauss;
        }
    }

    for i in -r..=r {
        for j in -r..=r {
            kernel[(i+r) as usize][(j+r) as usize] /= sum;
        }
    }

    return kernel;
}

pub fn abs_diff_graylevel_heatmap(pixmap1: &tiny_skia::Pixmap, pixmap2: &tiny_skia::Pixmap) -> tiny_skia::Pixmap {
    if pixmap1.width() != pixmap2.width() || pixmap1.height() != pixmap2.height() {
        panic!("Can not get difference of two images of different dimensions.");
    }

    let (mut canvas, width, height) = get_canvas(&pixmap1);
    let data = canvas.pixels_mut();

    let max_diff = 255.0 * 3.0;
    for i in 0..width {
        for j in 0..height {
            let c1 = pixmap1.pixel(i as u32, j as u32).expect("Could not get pixel. Checked before, impossible").demultiply();
            let c2 = pixmap2.pixel(i as u32, j as u32).expect("Could not get pixel. Checked before, impossible").demultiply();
            let r = (c1.red() as i32 - c2.red() as i32).abs() as f64;
            let g = (c1.green() as i32 - c2.green() as i32).abs() as f64;
            let b = (c1.blue() as i32 - c2.blue() as i32).abs() as f64;

            let gray = (255.0 * (r + g + b)/max_diff) as u8;

            let idx = (j * width + i) as usize;
            data[idx] = tiny_skia::PremultipliedColorU8::from_rgba(gray, gray, gray, 255).unwrap();
        }
    }

    return canvas;
}

pub struct GraylevelMask {
    dist: rand_distr::WeightedIndex<f64>,
    width: u32,
    height: u32,
}

impl GraylevelMask {

    pub fn from(src: &tiny_skia::Pixmap) -> Self {
        let gray = src.pixels().iter().map(|p| rgba_to_grayscale(p) as f64).collect::<Vec<f64>>();
        let sum = gray.iter().sum::<f64>();
        if sum == 0.0 {
            panic!("Cant use all black image as mask.");
        }
        let weights = gray.iter().map(|v| v/sum).collect::<Vec<f64>>();
        Self {
            dist: rand_distr::WeightedIndex::new(&weights).unwrap(),
            width: src.width(),
            height: src.height(),
        }
    }

    pub fn sample_random_i(&self) -> u32 {
        let mut rng = rand::thread_rng();
        self.dist.sample(&mut rng) as u32
    }

    pub fn sample_random_xy(&self) -> (u32, u32) {
        let i = self.sample_random_i();
        ((i as f64 / self.width as f64).floor() as u32, i % self.width)
    }

    pub fn dim(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
