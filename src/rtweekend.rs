use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_float() -> f64 {
    random_float_range(0.0, 1.0)
}

pub fn random_float_range(range_min: f64, range_max: f64) -> f64 {
    rand::thread_rng().gen_range(range_min..range_max)
}
