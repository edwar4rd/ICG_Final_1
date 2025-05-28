pub type Vec3 = nalgebra::Vector3<f64>;
pub type Point3 = Vec3;

pub fn random_vec3() -> Vec3 {
    use rand::Rng;
    Vec3::new(
        rand::rng().random_range(0.0..1.),
        rand::rng().random_range(0.0..1.),
        rand::rng().random_range(0.0..1.),
    )
}

pub fn random_vec3_in<R: rand::distr::uniform::SampleRange<f64> + Clone>(range: R) -> Vec3 {
    use rand::Rng;
    Vec3::new(
        rand::rng().random_range(range.clone()),
        rand::rng().random_range(range.clone()),
        rand::rng().random_range(range.clone()),
    )
}

pub fn random_unit_vec3() -> Vec3 {
    loop {
        let v = random_vec3_in(-1.0..=1.0);
        let v_squared = v.norm_squared();
        if 1e-160 < v_squared && v_squared <= 1.0 {
            break v.normalize();
        }
    }
}

pub fn random_vec3_in_unit_disk() -> Vec3 {
    use rand::Rng;

    loop {
        let v = Vec3::new(
            rand::rng().random_range(-1.0..=1.0),
            rand::rng().random_range(-1.0..=1.0),
            0.0,
        );
        if v.norm_squared() <= 1.0 {
            break v;
        }
    }
}

pub fn random_vec3_on_hemisphere(normal: Vec3) -> Vec3 {
    let v = random_unit_vec3();
    if v.dot(&normal) > 0.0 { v } else { -v }
}

pub fn near_zero(v: &Vec3) -> bool {
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

pub mod camera;
pub mod color;
pub mod ray;
pub use ray::Ray;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod sphere;
