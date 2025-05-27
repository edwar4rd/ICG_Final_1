use icg_final_1::{Point3, camera::Camera};
use std::io::stdout;

fn main() {
    env_logger::init();
    Camera::new(1.0, 400, 16.0 / 9.0, 2.0, Point3::new(0.0, 0.0, 0.0))
        .render(&mut stdout())
        .unwrap();
}
