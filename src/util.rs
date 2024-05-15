
pub fn bounded_add(a: u8, b: i64) -> u8 {
    if b > 0 {
        a.checked_add(b as u8).unwrap_or(u8::MAX)
    } else {
        a.checked_sub(-b as u8).unwrap_or(u8::MIN)
    }
}

pub fn bound<T: Ord>(value: T, min: T, max: T) -> T {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}