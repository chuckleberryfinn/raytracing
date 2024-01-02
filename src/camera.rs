use std::io::{stderr, Write};

use crate::colour;
use crate::hittable;
use crate::interval;
use crate::ray;
use crate::rtweekend;
use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i64,
    pub samples_per_pixel: i32,
    pub image_height: i64,
    pub max_depth: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i64, samples_per_pixel: i32, max_depth: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }

    pub fn render(&mut self, world: &impl hittable::Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            let progress = self.image_height - j;
            eprint!("\rScanlines remaining: {progress} ");
            stderr().flush().expect("Unable to flush stderr");
            for i in 0..self.image_width {
                let mut pixel_colour = colour::Colour::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += self.ray_colour(&r, self.max_depth, world);
                }
                colour::write_colour(&pixel_colour, self.samples_per_pixel);
            }
        }
        eprintln!("\rDone.                 ");
        stderr().flush().expect("Unable to flush stderr");
    }

    fn ray_colour(&self, r: &ray::Ray, depth: i32, world: &impl hittable::Hittable) -> colour::Colour {
        if depth <= 0 {
            return colour::Colour::new(0.0, 0.0, 0.0);
        }

        let mut rec = hittable::HitRecord::default();
        if world.hit(
            r,
            interval::Interval::new(0.0, rtweekend::INFINITY),
            &mut rec,
        ) {
            let direction = Vec3::random_on_hemisphere(rec.normal);
            return 0.5 * self.ray_colour(&ray::Ray::new(rec.p, direction), depth-1, world);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * colour::Colour::new(1.0, 1.0, 1.0) + (a * colour::Colour::new(0.5, 0.7, 1.0))
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i64;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width / self.image_height) as f64;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (0.5 * (self.pixel_delta_u + self.pixel_delta_v));
    }

    fn get_ray(&self, i: i64, j: i64) -> ray::Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        ray::Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 * rtweekend::random_float();
        let py = -0.5 * rtweekend::random_float();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
