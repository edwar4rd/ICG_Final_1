use icg_final_1::{
    Point3, camera::Camera, hittable::Hittable, hittable_list::HittableList, material::Lambertian,
    sphere::Sphere,
};
use std::{io::stdout, rc::Rc};

fn main() {
    env_logger::init();
    Camera::new(
        1.0,
        400,
        16.0 / 9.0,
        Point3::new(0.0, 0.0, 0.0),
        100,
        50,
        90.0,
    )
    .render(&mut stdout(), &create_world())
    .unwrap();
}

fn create_world() -> impl Hittable {
    let mut world = HittableList::new();
    let r = std::f64::consts::FRAC_PI_4.cos();
    let material_left = Lambertian::new(Point3::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Point3::new(1.0, 0.0, 0.0));

    world.push(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        Rc::new(material_left),
    ));

    world.push(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        Rc::new(material_right),
    ));

    world
}
