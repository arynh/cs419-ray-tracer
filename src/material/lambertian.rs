use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    fn random_direction() -> Vec3 {
        let theta = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let z = (rand::random::<f32>() * 2.0) - 1.0;
        let r = (1.0 - z * z).sqrt();
        glm::vec3(r * theta.cos(), r * theta.sin(), z)
    }
}

impl Material for Lambertian {
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let scatter_direction = hit_record.normal() + Lambertian::random_direction();
        Some(Ray {
            origin: hit_record.hit_point,
            direction: scatter_direction,
            attenuation: self.albedo,
        })
    }
}
