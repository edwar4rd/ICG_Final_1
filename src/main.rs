use icg_final_1::{
    Point3,
    camera::Camera,
    hittable::Hittable,
    hittable_list::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
};
use std::{io::stdout, rc::Rc};

fn main() {
    env_logger::init();
    Camera::new(
        1.0,
        400,
        16.0 / 9.0,
        2.0,
        Point3::new(0.0, 0.0, 0.0),
        100,
        50,
    )
    .render(&mut stdout(), &create_world())
    .unwrap();
}

fn create_world() -> impl Hittable {
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Point3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Point3::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Point3::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Point3::new(0.8, 0.6, 0.2));

    world.push(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    ));

    world.push(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Rc::new(material_center),
    ));

    world.push(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    ));

    world.push(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    ));

    world
}
