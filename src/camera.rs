use rayon::prelude::*;
use std::io::{stderr, Write};
use std::sync::Arc;

use crate::colour;
use crate::hittable;
use crate::interval;
use crate::material;
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
    vfov: f64,
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i64,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_dist,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
        }
    }

    pub fn render(&mut self, world: &(impl hittable::Hittable + std::marker::Sync)) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            let progress = self.image_height - j;
            eprint!("\rScanlines remaining: {progress} ");
            stderr().flush().expect("Unable to flush stderr");
            for i in 0..self.image_width {
                let pixel_colour = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .fold(
                        || colour::Colour::new(0.0, 0.0, 0.0),
                        |acc, item| {
                            acc + Camera::ray_colour(&self.get_ray(i, j), self.max_depth, world)
                        },
                    )
                    .reduce(
                        || colour::Colour::new(0.0, 0.0, 0.0),
                        |acc, item| acc + item,
                    );
                colour::write_colour(&pixel_colour, self.samples_per_pixel);
            }
        }
        eprintln!("\rDone.                 ");
        stderr().flush().expect("Unable to flush stderr");
    }

    fn ray_colour(r: &ray::Ray, depth: i32, world: &impl hittable::Hittable) -> colour::Colour {
        if depth <= 0 {
            return colour::Colour::new(0.0, 0.0, 0.0);
        }

        let mut rec = hittable::HitRecord {
            mat: Arc::new(material::Lambertian {
                albedo: colour::Colour::default(),
            }),
            p: Point3::default(),
            normal: Vec3::default(),
            front_face: bool::default(),
            t: f64::default(),
        };
        if world.hit(
            r,
            interval::Interval::new(0.001, rtweekend::INFINITY),
            &mut rec,
        ) {
            let mut scattered = ray::Ray::default();
            let mut attenuation = colour::Colour::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Camera::ray_colour(&scattered, depth - 1, world);
            }
            return colour::Colour::new(0.0, 0.0, 0.0);
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

        self.center = self.look_from;

        let theta = rtweekend::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = (self.vup.cross(self.w)).unit_vector();
        self.v = self.w.cross(self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (0.5 * (self.pixel_delta_u + self.pixel_delta_v));
        let defocus_radius =
            self.focus_dist * rtweekend::degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i64, j: i64) -> ray::Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        ray::Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 * rtweekend::random_float();
        let py = -0.5 * rtweekend::random_float();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }
}
