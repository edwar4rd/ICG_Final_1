use crate::Point3;
use crate::Ray;
use crate::Rc;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().magnitude_squared();
        let h = oc.dot(&ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !t_range.contains(&root) {
            root = (h + sqrt_d) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        Some(HitRecord::new(
            root,
            ray.at(root),
            self.mat.clone(),
            (ray.at(root) - self.center) / self.radius,
            ray,
        ))
    }
}
