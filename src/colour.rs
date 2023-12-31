use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_colour(pixel_colour: &Colour) {
    println!(
        "{} {} {}",
        (255.999 * pixel_colour.x()) as i64,
        (255.999 * pixel_colour.y()) as i64,
        (255.999 * pixel_colour.z()) as i64
    );
}
