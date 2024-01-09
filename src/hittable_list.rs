use std::sync::Arc;

use crate::colour::Colour;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Lambertian;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(item: T) -> Self {
        HittableList {
            objects: vec![item],
        }
    }

    pub fn add(&mut self, item: T) {
        self.objects.push(item);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max;

        for object in &self.objects {
            let mut temp_rec = HitRecord {
                mat: Arc::new(Lambertian {
                    albedo: Colour::default(),
                }),
                p: Point3::default(),
                normal: Vec3::default(),
                front_face: bool::default(),
                t: f64::default(),
            };
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
