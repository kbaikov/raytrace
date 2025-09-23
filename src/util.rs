use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    fastrand::f64()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
