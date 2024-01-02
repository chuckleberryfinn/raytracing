use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn random_float() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

fn random_float_range(range_min: f64, range_max: f64) -> f64 {
    rand::thread_rng().gen_range(range_min..range_max)
}
