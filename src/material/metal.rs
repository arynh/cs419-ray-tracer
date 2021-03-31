use super::super::EPSILON;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;
use rand::prelude::thread_rng as rng;
use rand::Rng;

/// Represent a metal material with reflection
pub struct Metal {
    /// Base albedo of the material
    pub albedo: Vec3,
}

/// Methods specific to metalic materials
impl Metal {}

/// Methods for the material trait
impl Material for Metal {
    /// Determine where the next ray goes after a hit depending on this
    /// material.
    ///
    /// # Arguments
    /// - self reference
    /// - `incoming_ray` - ray which has just hit this material
    /// - `hit_record` - specification of the hit which just ocurred
    ///
    /// # Returns
    /// - optional `Ray`, or none if the ray was absorbed
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let reflected_direction = glm::reflect_vec(&incoming_ray.direction, &hit_record.normal());
        if glm::dot(&reflected_direction, &hit_record.normal()) > 0.0 {
            Some(Ray::new(
                hit_record.hit_point,
                reflected_direction,
                Some(self.albedo),
            ))
        } else {
            None
        }
    }

    /// Retrieve the base color of the material.
    ///
    /// # Arguments
    /// - self reference
    ///
    /// # Returns
    /// - `Vec3` - the RGB color
    fn color(&self) -> Vec3 {
        self.albedo
    }
}
