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
    let material_ground = Rc::new(material::Lambertian {
        albedo: colour::Colour::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(material::Lambertian {
        albedo: colour::Colour::new(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(material::Metal::new(
        colour::Colour::new(0.8, 0.8, 0.8),
        0.3,
    ));
    let material_right = Rc::new(material::Metal::new(
        colour::Colour::new(0.8, 0.6, 0.2),
        1.0,
    ));
    let mut world = hittable_list::HittableList::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(sphere::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.add(sphere::Sphere::new(
        vec3::Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(sphere::Sphere::new(
        vec3::Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let mut cam = camera::Camera::new(16.0 / 9.0, 400, 100, 10);
    cam.render(&world);
}
