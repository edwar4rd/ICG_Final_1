use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn from_vec(objects: Vec<Rc<dyn Hittable>>) -> Self {
        HittableList { objects }
    }

    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn push<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Rc::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord> {
        let mut t_range = t_range.clone();
        let mut hit: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(new_hit) = object.hit(ray, &t_range) {
                hit = Some(new_hit);
                t_range.end = new_hit.t;
            }
        }

        hit
    }
}
