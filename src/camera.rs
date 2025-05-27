use crate::{
    Point3, Ray, Vec3,
    color::{Color, write_color},
};
use log::info;
use std::io::stdout;

#[derive(Debug, Clone)]
pub struct Camera {
    // focal_length: f64,
    image_width: usize,
    image_height: usize,
    // image_aspect_ratio: f64,
    // viewport_height: f64,
    // viewport_width: f64,
    camera_center: Point3,
    // viewport_u: Vec3,
    // viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // viewport_upper_left: Point3,
    pixel00_loc: Point3,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        image_width: usize,
        image_aspect_ratio: f64,
        viewport_height: f64,
        camera_center: Point3,
    ) -> Self {
        let image_height = (image_width as f64 / image_aspect_ratio).max(1.0) as usize;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);
        let viewport_upper_left =
            camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            // focal_length,
            image_width,
            image_height,
            // image_aspect_ratio,
            // viewport_height,
            // viewport_width,
            camera_center,
            // viewport_u,
            // viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            // viewport_upper_left,
            pixel00_loc,
        }
    }

    pub fn render<T: std::io::Write>(&self, file: &mut T) -> std::io::Result<()> {
        writeln!(file, "P3\n{} {}", self.image_width, self.image_height)?;
        println!("255");
        for y in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - y);
            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (x as f64 * self.pixel_delta_u)
                    + (y as f64 * self.pixel_delta_v);
                let pixel_dir = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, pixel_dir);
                let color = ray_color(&ray);

                write_color(&mut stdout(), color)?;
            }
        }
        info!("Done.");
        Ok(())
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = ray.direction().magnitude_squared();
    let h = oc.dot(&ray.direction());
    let c = oc.magnitude_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        // Hit the sphere
        let normal = (ray.at(t) - Point3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    let color_a = Color::new(1.0, 1.0, 1.0);
    let color_b = Color::new(0.5, 0.7, 1.0);
    let unit_direction = ray.direction().normalize();
    let tt = 0.5 * (unit_direction.y + 1.0);
    (1.0 - tt) * color_a + tt * color_b
}
