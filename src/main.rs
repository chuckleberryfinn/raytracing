use std::io::{stderr, Write};
mod colour;
mod vec3;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in 0..IMAGE_HEIGHT {
        let progress = IMAGE_HEIGHT - j;
        eprint!("\rScanlines remaining: {progress} ");
        stderr().flush().expect("Unable to flush stderr");
        for i in 0..IMAGE_WIDTH {
            let pixel_colour = colour::Colour::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );
            colour::write_colour(pixel_colour);
        }
    }
    eprintln!("\rDone.                 ");
    stderr().flush().expect("Unable to flush stderr");
}
