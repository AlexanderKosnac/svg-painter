use std::path::Path;
use rand::Rng;
use std::collections::HashSet;

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

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn random_points_in_range(n: u64, start: u64, end: u64) -> Vec<u64> {
    let span = end - start;
    let n_safe = std::cmp::min(span, n);
    if span == n_safe {
        return (start..end).collect::<Vec<u64>>();
    }

    let mut rng = rand::thread_rng();
    let mut numbers: HashSet<u64> = HashSet::new();
    numbers.insert(start);
    numbers.insert(end);

    while (numbers.len() as u64) < n_safe {
        numbers.insert(rng.gen_range(start..end));
    }

    let mut points: Vec<u64> = numbers.into_iter().collect();
    points.sort();
    points
}

pub fn read_image(path: &String) -> tiny_skia::Pixmap {
    return tiny_skia::Pixmap::load_png(&Path::new(path)).expect("Failed to open image");
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
    if c1.alpha() == c2.alpha() {
        let dr = c1.red() as i32 - c2.red() as i32;
        let dg = c1.green() as i32 - c2.green() as i32;
        let db = c1.blue() as i32 - c2.blue() as i32;
        ((dr*dr + dg*dg + db*db) as f64).sqrt() as f64
    } else {
        ((255.0*255.0*3.0) as f64).sqrt()
    }
}
