use icg_final_1::{
    Point3, camera::Camera, hittable::Hittable, hittable_list::HittableList, sphere::Sphere,
};
use std::io::stdout;

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
    world.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    world
}
