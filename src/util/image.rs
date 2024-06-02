use tiny_skia;

use crate::util;

// See: https://en.wikipedia.org/wiki/Gaussian_blur#Sample_Gaussian_matrix
static GAUSSIAN_BLUR_KERNEL: [[f64; 7]; 7] = [
    [0.00000067, 0.00002292, 0.00019117, 0.00038771, 0.00019117, 0.00002292, 0.00000067],
    [0.00002292, 0.00078633, 0.00655965, 0.01330373, 0.00655965, 0.00078633, 0.00002292],
    [0.00019117, 0.00655965, 0.05472157, 0.11098164, 0.05472157, 0.00655965, 0.00019117],
    [0.00038771, 0.01330373, 0.11098164, 0.22508352, 0.11098164, 0.01330373, 0.00038771],
    [0.00019117, 0.00655965, 0.05472157, 0.11098164, 0.05472157, 0.00655965, 0.00019117],
    [0.00002292, 0.00078633, 0.00655965, 0.01330373, 0.00655965, 0.00078633, 0.00002292],
    [0.00000067, 0.00002292, 0.00019117, 0.00038771, 0.00019117, 0.00002292, 0.00000067],
];

pub fn sobel(input: tiny_skia::Pixmap) -> tiny_skia::Pixmap {
    let def = tiny_skia::PremultipliedColorU8::from_rgba(0, 0, 0, 255).unwrap();

    let width = input.width() as i32;
    let height = input.height() as i32;

    let mut new = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
    let mut data = new.pixels_mut();

    let get_pixel = |x: i32, y: i32| -> i32 {
        let q = if x < 0 || y < 0 { def } else { input.pixel(x as u32, y as u32).unwrap_or(def) };
        let p = q.demultiply();
        let grey = p.red() as f64 * 0.299 + p.green() as f64 * 0.587 + p.blue() as f64 * 0.114;
        grey as i32
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

    return new;
}

pub fn gaussian_blur(input: tiny_skia::Pixmap) -> tiny_skia::Pixmap {
    let def = tiny_skia::PremultipliedColorU8::from_rgba(0, 0, 0, 255).unwrap();

    let width = input.width() as i32;
    let height = input.height() as i32;
    let kernel_width = GAUSSIAN_BLUR_KERNEL[0].len() as i32;
    let kernel_height = GAUSSIAN_BLUR_KERNEL.len() as i32;
    let kernel_width_offset = (kernel_width as f64/2.0).floor() as i32;
    let kernel_height_offset = (kernel_height as f64/2.0).floor() as i32;

    let mut new = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
    let mut data = new.pixels_mut();

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
                    c[0] += GAUSSIAN_BLUR_KERNEL[idcs.0][idcs.1] * p[0];
                    c[1] += GAUSSIAN_BLUR_KERNEL[idcs.0][idcs.1] * p[1];
                    c[2] += GAUSSIAN_BLUR_KERNEL[idcs.0][idcs.1] * p[2];
                    c[3] += GAUSSIAN_BLUR_KERNEL[idcs.0][idcs.1] * p[3];
                }
            }

            let idx = (j * width + i) as usize;
            data[idx] = tiny_skia::PremultipliedColorU8::from_rgba(c[0] as u8, c[1] as u8, c[2] as u8, c[3] as u8).unwrap();
        }
    }

    return new;
}

