use rand::random;

use crate::{Ray, color::Color, hittable::HitRecord, near_zero, reflect, refract};

#[cfg(feature = "rayon")]
pub trait Material: std::fmt::Debug + Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

#[cfg(not(feature = "rayon"))]
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
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().normalize();

        let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > random() {
            reflect(&unit_direction, &hit_record.normal)
        } else {
            refract(&unit_direction, &hit_record.normal, ri)
        };

        let scattered = Ray::new(hit_record.p, direction);
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Debug, Clone, Copy)]
pub struct Portal {
    albedo: Color,
    portal_position: crate::Point3,
    target_position: crate::Point3,
}

impl Portal {
    pub fn new(
        albedo: Color,
        portal_position: crate::Point3,
        target_position: crate::Point3,
    ) -> Self {
        Portal {
            albedo,
            portal_position,
            target_position,
        }
    }

    pub fn new_pair(
        albedo_a: Color,
        albedo_b: Color,
        pos_a: crate::Point3,
        pos_b: crate::Point3,
    ) -> (Self, Self) {
        (
            Portal::new(albedo_a, pos_a, pos_b),
            Portal::new(albedo_b, pos_b, pos_a),
        )
    }
}

impl Material for Portal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        if hit_record.front_face {
            // The ray is entering the portal
            return Some((
                Color::new(1.0, 1.0, 1.0),
                Ray::new(hit_record.p, ray_in.direction()),
            ));
        }

        let scattered = Ray::new(
            self.target_position + (hit_record.p - self.portal_position),
            ray_in.direction(),
        );
        Some((self.albedo, scattered))
    }
}
