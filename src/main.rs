use std::rc::Rc;

mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let R = (rtweekend::PI / 4.0).cos();
    let material_left = Rc::new(material::Lambertian {
        albedo: colour::Colour::new(0.0, 0.0, 1.0),
    });
    let material_right = Rc::new(material::Lambertian {
        albedo: colour::Colour::new(1.0, 0.0, 0.0),
    });
    let mut world = hittable_list::HittableList::new(sphere::Sphere::new(
        vec3::Point3::new(-R, 0.0, -1.0),
        R,
        material_left,
    ));
    world.add(sphere::Sphere::new(
        vec3::Point3::new(R, 0.0, -1.0),
        R,
        material_right,
    ));

    let mut cam = camera::Camera::new(16.0 / 9.0, 400, 100, 50, 90.0);
    cam.render(&world);
}
