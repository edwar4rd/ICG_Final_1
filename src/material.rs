use crate::{Ray, color::Color, hittable::HitRecord, near_zero};

pub trait Material: std::fmt::Debug {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = hit_record.normal + crate::random_unit_vec3();
        if near_zero(&scatter_dir) {
            scatter_dir = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.p, scatter_dir);
        Some((self.albedo, scattered))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected_dir = crate::reflect(&ray_in.direction(), &hit_record.normal).normalize();
        let scattered = Ray::new(
            hit_record.p,
            reflected_dir + self.fuzz * crate::random_unit_vec3(),
        );
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
