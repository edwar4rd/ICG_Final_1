use std::ops::Range;

use crate::Ray;
use crate::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord>;
}
