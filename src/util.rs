use std::path::Path;

use resvg;
use usvg;
use fontdb;
use tiny_skia;
use tiny_skia_path;

pub mod image;

pub fn bounded_add(a: u8, b: i64) -> u8 {
    if b > 0 {
        a.checked_add(b as u8).unwrap_or(u8::MAX)
    } else {
        a.checked_sub(-b as u8).unwrap_or(u8::MIN)
    }
}

pub fn clamp<T: Ord>(value: T, min: T, max: T) -> T {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}

pub fn read_image(path: &String) -> tiny_skia::Pixmap {
    let path_obj = Path::new(path);
    return tiny_skia::Pixmap::load_png(&path_obj).expect("Failed to open image");
}

pub fn render_svg_into_pixmap(svg_data: &String, pixmap: &mut tiny_skia::Pixmap) {
    let opt = usvg::Options::default();
    let db = fontdb::Database::new();

    let tree = usvg::Tree::from_str(&svg_data, &opt, &db).unwrap();
    let transformation = tiny_skia_path::Transform::identity();
    resvg::render(&tree, transformation, &mut pixmap.as_mut());
}

pub fn pixmap_distance(d1: &tiny_skia::Pixmap, d2: &tiny_skia::Pixmap) -> f64 {
    if d1.width() != d2.width() || d1.height() != d2.height() {
        panic!("Pixmaps of different dimensions can not be compared. Got {}x{} and {}x{}.", d1.width(), d1.height(), d2.width(), d2.height());
    }
    let pixels1 = d1.pixels();
    let pixels2 = d2.pixels();
    return (0..pixels1.len()).map(|i| coloru8_norm_2(&pixels1[i], &pixels2[i]) as f64).map(|i| i*i).sum::<f64>().sqrt();
}

fn coloru8_norm_2(c1: &tiny_skia::PremultipliedColorU8, c2: &tiny_skia::PremultipliedColorU8) -> f64 {
    let dr = c1.red() as i32 - c2.red() as i32;
    let dg = c1.green() as i32 - c2.green() as i32;
    let db = c1.blue() as i32 - c2.blue() as i32;
    let da = c1.alpha() as i32 - c2.alpha() as i32;
    return ((dr*dr + dg*dg + db*db + da*da) as f64).sqrt() as f64;
}
