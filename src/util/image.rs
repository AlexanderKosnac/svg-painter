use tiny_skia;

use crate::util;

pub fn sobel(input: tiny_skia::Pixmap) -> tiny_skia::Pixmap {
    let def = tiny_skia::PremultipliedColorU8::from_rgba(0, 0, 0, 255).unwrap(); // Black

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
