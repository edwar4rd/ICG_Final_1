use icg_final_1::{
    Point3, Rc,
    camera::{Camera, CameraSettings, ImageSettings, QualitySettings},
    color::Color,
    hittable::Hittable,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
};
use std::io::stdout;

fn main() {
    env_logger::init();
    let image_settings = ImageSettings {
        image_width: 1200,
        aspect_ratio: 16.0 / 9.0,
    };
    let quality_settings = QualitySettings {
        samples_per_pixel: 500,
        max_depth: 50,
    };
    let camera_settings = CameraSettings {
        vfov: 20.0,
        focus_dist: 10.0,
        defocus_angle: 0.6,
        camera_center: Point3::new(13.0, 2.0, 3.0),
        camera_lookat: Point3::new(0.0, 0.0, 0.0),
        camera_vup: Point3::new(0.0, 1.0, 0.0),
    };

    Camera::new(image_settings, quality_settings, camera_settings)
        .par_render(&mut stdout(), &create_world())
        .unwrap();
}

fn create_world() -> impl Hittable {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(ground_material),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand::random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::new(
                        rand::random::<f64>() * rand::random::<f64>(),
                        rand::random::<f64>() * rand::random::<f64>(),
                        rand::random::<f64>() * rand::random::<f64>(),
                    );
                    world.push(Sphere::new(center, 0.2, Rc::new(Lambertian::new(albedo))));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::new(
                        0.5 * (1.0 + rand::random::<f64>()),
                        0.5 * (1.0 + rand::random::<f64>()),
                        0.5 * (1.0 + rand::random::<f64>()),
                    );
                    let fuzz = 0.5 * rand::random::<f64>();
                    world.push(Sphere::new(center, 0.2, Rc::new(Metal::new(albedo, fuzz))));
                } else {
                    // glass
                    world.push(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5))));
                }
            }
        }
    }

    world.push(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    ));

    world.push(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    ));

    world.push(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    ));

    world
}
