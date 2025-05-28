use crate::{Ray, color::Color, hittable::HitRecord, near_zero, refract};

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

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric {
            refraction_index: refraction_index.max(1.0),
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected_dir = crate::reflect(&ray_in.direction(), &hit_record.normal).normalize();
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().normalize();
        let refracted = refract(&unit_direction, &hit_record.normal, ri);
        let scattered = Ray::new(hit_record.p, refracted);
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
