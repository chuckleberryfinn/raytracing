use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

struct HittableList<T: Hittable> {
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
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_tmax;

        for object in &self.objects {
            let mut temp_rec = HitRecord::default();
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
