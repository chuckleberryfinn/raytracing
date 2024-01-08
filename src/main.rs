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
    let ground_material = Rc::new(material::Lambertian {
        albedo: colour::Colour::new(0.5, 0.5, 0.5),
    });

    let mut world = hittable_list::HittableList::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_float();
            let center = vec3::Point3::new(
                a as f64 + 0.9 * rtweekend::random_float(),
                0.2,
                b as f64 + 0.9 * rtweekend::random_float(),
            );

            if (center - vec3::Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = colour::Colour::random() * colour::Colour::random();
                    let sphere_material = Rc::new(material::Lambertian { albedo });
                    world.add(sphere::Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = colour::Colour::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_float_range(0.0, 0.5);
                    let sphere_material = Rc::new(material::Metal { albedo, fuzz });
                    world.add(sphere::Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Rc::new(material::Dielectric { ir: 1.5 });
                    world.add(sphere::Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Rc::new(material::Dielectric { ir: 1.5 });
    world.add(sphere::Sphere::new(
        vec3::Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));

    let material2 = Rc::new(material::Lambertian {
        albedo: colour::Colour::new(0.4, 0.2, 0.1),
    });
    world.add(sphere::Sphere::new(
        vec3::Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Rc::new(material::Metal {
        albedo: colour::Colour::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(sphere::Sphere::new(
        vec3::Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));

    let mut cam = camera::Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        vec3::Point3::new(13.0, 2.0, 3.0),
        vec3::Point3::new(0.0, 0.0, 0.0),
        vec3::Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );
    cam.render(&world);
}
