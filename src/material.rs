pub mod lambertian;

use crate::hit_record::HitRecord;
use crate::ray::Ray;
use glm::Vec3;

pub trait Material {
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord) -> Option<Ray>;
    fn color(&self) -> Vec3;
}
