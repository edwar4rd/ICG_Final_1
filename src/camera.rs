use crate::{
    Point3, Ray, Vec3,
    color::{Color, write_color},
    hittable::Hittable,
};
use log::info;
use rand::Rng;
use std::io::stdout;

#[derive(Debug, Clone)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    // image_aspect_ratio: f64,
    camera_center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    samples_per_pixel: usize,
    pixel_samples_scale: f64,
    max_depth: usize,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        image_width: usize,
        image_aspect_ratio: f64,
        camera_center: Point3,
        samples_per_pixel: usize,
        max_depth: usize,
        vfov: f64,
    ) -> Self {
        let image_height = (image_width as f64 / image_aspect_ratio).max(1.0) as usize;
        let viewport_height = 2.0 * (focal_length * (vfov.to_radians() / 2.0).tan());
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);
        let viewport_upper_left =
            camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let pixel_samples_scale = (samples_per_pixel as f64).recip();

        Camera {
            image_width,
            image_height,
            // image_aspect_ratio,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn render<T: std::io::Write, W: Hittable>(
        &self,
        file: &mut T,
        world: &W,
    ) -> std::io::Result<()> {
        writeln!(file, "P3\n{} {}", self.image_width, self.image_height)?;
        println!("255");
        for y in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - y);
            for x in 0..self.image_width {
                let mut color = Color::zeros();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += ray_color(&ray, world, self.max_depth);
                }
                write_color(&mut stdout(), color * self.pixel_samples_scale)?;
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
        let pixel_dir = sample_center - self.camera_center;
        Ray::new(self.camera_center, pixel_dir)
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
