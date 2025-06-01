use crate::Point3;
use crate::Ray;
use crate::Rc;
use crate::Vec3;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct Disk {
    center: Point3,
    normal: Vec3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Disk {
    pub const fn new(center: Point3, normal: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Disk {
            center,
            normal,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Disk {
    fn hit(&self, ray: &Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord> {
        let d = self.normal.dot(&self.center);
        let denom = self.normal.dot(&ray.direction());
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (d - self.normal.dot(&ray.origin())) / denom;
        if !t_range.contains(&t) {
            return None;
        }

        let intersection = ray.at(t);
        if (intersection - self.center).magnitude_squared() > self.radius * self.radius {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.at(t),
            self.mat.clone(),
            self.normal,
            ray,
        ))
    }
}
