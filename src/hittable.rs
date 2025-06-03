use crate::Rc;
use std::ops::Range;

use crate::Ray;
use crate::Vec3;
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub mat: Rc<dyn Material>,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, mat: Rc<dyn Material>, outward_normal: Vec3, ray: &Ray) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            t,
            p,
            mat,
            normal,
            front_face,
        }
    }
}

#[cfg(feature = "rayon")]
pub trait Hittable: std::fmt::Debug + Send + Sync {
    fn hit(&self, ray: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}

#[cfg(not(feature = "rayon"))]
pub trait Hittable: std::fmt::Debug {
    fn hit(&self, ray: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}
