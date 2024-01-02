mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut world = hittable_list::HittableList::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
    ));
    world.add(sphere::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
    ));

    let mut cam = camera::Camera::new(16.0 / 9.0, 400, 100);
    cam.render(&world);
}
