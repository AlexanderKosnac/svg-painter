use rand::Rng;
use crate::util;

pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r: r, g: g, b: b, a: a }
    }

    pub fn new_rand() -> Rgba {
        let mut rng = rand::thread_rng();
        Rgba::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))
    }

    pub fn as_hex(&self) -> String {
        return format!("#{:X}{:X}{:X}", self.r, self.g, self.b);
    }

    pub fn mutate(&mut self, magnitude: f64) {
        let mut rng = rand::thread_rng();
        let dir_vec: (f64, f64, f64, f64) = (
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        );
        let len = (dir_vec.0.powf(2.0) + dir_vec.1.powf(2.0) + dir_vec.2.powf(2.0) + dir_vec.2.powf(2.0)).sqrt();
        let normed = (dir_vec.0/len, dir_vec.1/len, dir_vec.2/len, dir_vec.3/len);
        let scaled = (normed.0 * magnitude, normed.1 * magnitude, normed.2 * magnitude, normed.3 * magnitude);

        self.r = util::bounded_add(self.r, scaled.0 as i64);
        self.g = util::bounded_add(self.g, scaled.1 as i64);
        self.b = util::bounded_add(self.b, scaled.2 as i64);
        self.a = util::bounded_add(self.a, scaled.3 as i64);
    }
}

impl Clone for Rgba {
    fn clone(&self) -> Self {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}