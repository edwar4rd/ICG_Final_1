use icg_final_1::{
    Point3,
    camera::{Camera, CameraSettings, ImageSettings, QualitySettings},
    hittable::Hittable,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
};
use std::{io::stdout, rc::Rc};

fn main() {
    env_logger::init();
    let image_settings = ImageSettings {
        image_width: 400,
        aspect_ratio: 16.0 / 9.0,
    };
    let quality_settings = QualitySettings {
        samples_per_pixel: 100,
        max_depth: 50,
    };
    let camera_settings = CameraSettings {
        vfov: 20.0,
        focus_dist: 3.4,
        defocus_angle: 10.0,
        camera_center: Point3::new(-2.0, 2.0, 1.0),
        camera_lookat: Point3::new(0.0, 0.0, -1.0),
        camera_vup: Point3::new(0.0, 1.0, 0.0),
    };

    Camera::new(image_settings, quality_settings, camera_settings)
        .render(&mut stdout(), &create_world())
        .unwrap();
}

fn create_world() -> impl Hittable {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Point3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Point3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Point3::new(0.8, 0.6, 0.2), 1.0);

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
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Rc::new(material_bubble),
    ));

    world.push(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    ));

    world
}
