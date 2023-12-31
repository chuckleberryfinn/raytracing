use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub trait hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, ) -> Bool;
}
