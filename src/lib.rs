pub type Vec3 = nalgebra::Vector3<f64>;
pub type Point3 = Vec3;

pub mod camera;
pub mod color;
pub mod ray;
pub use ray::Ray;
pub mod hittable;
pub mod sphere;
