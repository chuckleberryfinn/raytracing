use std::io::{stderr, Write};
mod colour;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

const IMAGE_WIDTH: i64 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn hit_sphere(center: &vec3::Point3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius.powf(2.0);
    let discriminant = half_b.powf(2.0) - (a * c);

    if discriminant < 0.0 {
        return -1.0;
    }
    (-half_b - discriminant.sqrt()) / a
}

fn ray_colour(r: &ray::Ray) -> colour::Colour {
    let t = hit_sphere(&vec3::Point3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = (r.at(t) - vec3::Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * colour::Colour::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * colour::Colour::new(1.0, 1.0, 1.0) + (a * colour::Colour::new(0.5, 0.7, 1.0))
}

fn main() {
    let mut image_height: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH / image_height) as f64;
    let camera_center = vec3::Point3::new(0.0, 0.0, 0.0);

    let viewport_u = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
        - vec3::Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (0.5 * (pixel_delta_u + pixel_delta_v));

    println!("P3\n{IMAGE_WIDTH} {image_height}\n255");
    for j in 0..image_height {
        let progress = image_height - j;
        eprint!("\rScanlines remaining: {progress} ");
        stderr().flush().expect("Unable to flush stderr");
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = ray::Ray::new(camera_center, ray_direction);

            let pixel_colour = ray_colour(&r);
            colour::write_colour(&pixel_colour);
        }
    }
    eprintln!("\rDone.                 ");
    stderr().flush().expect("Unable to flush stderr");
}
