use icg_final_1::{
    Point3, Rc, Vec3,
    camera::{Camera, CameraSettings, ImageSettings, QualitySettings},
    color::Color,
    disk::Disk,
    hittable_list::HittableList,
    material::{Checker, Dielectric, Lambertian, Metal},
    sphere::Sphere,
};
use std::env::args;

fn main() {
    use rand::SeedableRng;
    env_logger::init();
    let image_settings = ImageSettings {
        image_width: 1200,
        aspect_ratio: 16.0 / 9.0,
    };
    let quality_settings = QualitySettings {
        samples_per_pixel: 500,
        max_depth: 400,
    };
    let camera_settings = CameraSettings {
        vfov: 20.0,
        focus_dist: 10.0,
        defocus_angle: 0.6,
        camera_center: Point3::new(15.0, 2.0, 3.0),
        camera_lookat: Point3::new(0.0, 0.0, 0.0),
        camera_vup: Point3::new(0.0, 1.0, 0.0),
    };

    let camera = Camera::new(image_settings, quality_settings, camera_settings);
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let world = match args().nth(1).as_deref() {
        Some("world") => create_world(&mut rng),
        Some("world2") => create_world_2(&mut rng),
        _ => {
            eprintln!("Usage: cargo run [world|world2]");
            return;
        }
    };

    #[cfg(feature = "image")]
    camera.render_to_imgbuf(&world).save("image.png").unwrap();

    #[cfg(not(feature = "image"))]
    camera.render(&mut std::io::stdout(), &world).unwrap();
}

fn create_world(rng: &mut impl rand::Rng) -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Checker {};
    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(ground_material),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::new(
                        rng.random::<f64>() * rng.random::<f64>(),
                        rng.random::<f64>() * rng.random::<f64>(),
                        rng.random::<f64>() * rng.random::<f64>(),
                    );
                    world.push(Sphere::new(center, 0.2, Rc::new(Lambertian::new(albedo))));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::new(
                        0.5 * (1.0 + rng.random::<f64>()),
                        0.5 * (1.0 + rng.random::<f64>()),
                        0.5 * (1.0 + rng.random::<f64>()),
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

    let (portal_left_material, portal_right_material) = icg_final_1::material::Portal::new_pair(
        Color::new(1.0, 0.5, 0.5),
        Color::new(0.5, 0.5, 1.0),
        Point3::new(-4.0, 1.0, 0.0),
        Point3::new(4.0, 1.0, 0.0),
    );

    world.push(Sphere::new(
        Point3::new(-8.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.4))),
    ));

    world.push(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(portal_left_material),
    ));

    world.push(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(portal_right_material),
    ));

    add_blackhole(Point3::new(8.0, 1.0, 0.0), &mut world, 1.0);

    world
}

fn create_world_2(_rng: &mut impl rand::Rng) -> HittableList {
    let mut world = HittableList::new();

    add_blackhole(Point3::new(0.0, 0.0, 0.0), &mut world, 3.0);
    world.push(Disk::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.21),
        4.0,
        Rc::new(Checker::new()),
    ));

    world
}

fn add_blackhole(position: Point3, world: &mut HittableList, scale: f64) {
    use icg_final_1::material::{Black, BlackHoleLayer};
    const LAYER_COUNT: usize = 64;

    for layer_index in 0..LAYER_COUNT {
        let radius = (layer_index as f64 / (LAYER_COUNT as f64 / 4.25)).powf(2.5) + 1.0;
        world.push(Sphere::new(
            position,
            radius / 40.0 * scale,
            Rc::new(BlackHoleLayer::new(radius, LAYER_COUNT as f64)),
        ));
    }

    world.push(Sphere::new(position, 0.01, Rc::new(Black::new())));
}
