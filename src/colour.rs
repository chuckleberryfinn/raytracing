use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_colour(pixel_colour: &Colour, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_colour.x() * scale;
    let g = pixel_colour.y() * scale;
    let b = pixel_colour.z() * scale;

    let intensity = Interval::new(0.000, 0.999);

    println!(
        "{} {} {}",
        (255.999 * intensity.clamp(r)) as i64,
        (255.999 * intensity.clamp(g)) as i64,
        (255.999 * intensity.clamp(b)) as i64
    );
}
