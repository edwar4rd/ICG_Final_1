use crate::{
    Point3, Ray, Vec3,
    color::{Color, write_color},
    hittable::Hittable,
    random_vec3_in_unit_disk,
};
use log::info;
use rand::Rng;
use std::io::stdout;

#[derive(Debug, Clone, Copy)]
pub struct ImageSettings {
    pub image_width: usize,
    pub aspect_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct QualitySettings {
    pub samples_per_pixel: usize,
    pub max_depth: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraSettings {
    pub vfov: f64,
    pub focus_dist: f64,
    pub defocus_angle: f64,
    pub camera_center: Point3,
    pub camera_lookat: Point3,
    pub camera_vup: Vec3,
}

#[derive(Debug, Clone)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    camera_center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    samples_per_pixel: usize,
    max_depth: usize,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        image_settings: ImageSettings,
        quality_settings: QualitySettings,
        camera_settings: CameraSettings,
    ) -> Self {
        let image_width = image_settings.image_width;
        let aspect_ratio = image_settings.aspect_ratio;
        let image_height = (image_width as f64 / aspect_ratio).max(1.0) as usize;
        let viewport_height =
            2.0 * (camera_settings.focus_dist * (camera_settings.vfov.to_radians() / 2.0).tan());
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (camera_settings.camera_center - camera_settings.camera_lookat).normalize();
        let u = camera_settings.camera_vup.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);
        let viewport_upper_left = camera_settings.camera_center
            - camera_settings.focus_dist * w
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            (camera_settings.defocus_angle / 2.0).to_radians().tan() * camera_settings.focus_dist;
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            camera_center: camera_settings.camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel: quality_settings.samples_per_pixel,
            max_depth: quality_settings.max_depth,
            defocus_angle: camera_settings.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render<T: std::io::Write, W: Hittable>(
        &self,
        file: &mut T,
        world: &W,
    ) -> std::io::Result<()> {
        writeln!(file, "P3\n{} {}", self.image_width, self.image_height)?;
        println!("255");
        let pixel_samples_scale = (self.samples_per_pixel as f64).recip();
        for y in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - y);
            for x in 0..self.image_width {
                let mut color = Color::zeros();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += ray_color(&ray, world, self.max_depth);
                }
                write_color(&mut stdout(), color * pixel_samples_scale)?;
            }
        }
        info!("Done.");
        Ok(())
    }
}

impl Camera {
    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let (offset_x, offset_y) = (
            rand::rng().random_range(-0.5..0.5),
            rand::rng().random_range(-0.5..0.5),
        );

        let sample_center = self.pixel00_loc
            + ((x as f64 + offset_x) * self.pixel_delta_u)
            + ((y as f64 + offset_y) * self.pixel_delta_v);
        let origin = if self.defocus_angle != 0.0 {
            let p = random_vec3_in_unit_disk();
            self.camera_center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
        } else {
            self.camera_center
        };
        let dir = sample_center - origin;
        Ray::new(origin, dir)
    }
}

fn ray_color<W: Hittable>(ray: &Ray, world: &W, depth: usize) -> Color {
    if depth == 0 {
        return Color::zeros();
    }

    if let Some(hit) = world.hit(ray, &(0.001..f64::INFINITY)) {
        if let Some((attenuation, scattered)) = hit.mat.scatter(ray, &hit) {
            attenuation.component_mul(&ray_color(&scattered, world, depth - 1))
        } else {
            Color::zeros()
        }
    } else {
        let color_a = Color::new(1.0, 1.0, 1.0);
        let color_b = Color::new(0.5, 0.7, 1.0);
        let unit_direction = ray.direction().normalize();
        let tt = 0.5 * (unit_direction.y + 1.0);
        (1.0 - tt) * color_a + tt * color_b
    }
}
