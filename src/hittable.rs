use std::ops::Range;

use crate::Ray;
use crate::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, outward_normal: Vec3, ray: &Ray) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            t,
            p,
            normal,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}
