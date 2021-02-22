pub mod lambertian;

use crate::hit_record::HitRecord;
use crate::ray::Ray;
use glm::Vec3;

/// Material trait
pub trait Material {
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
    fn scatter(&self, incoming_ray: &Ray, hit_record: &HitRecord) -> Option<Ray>;

    /// Retrieve the base color of the material.
    ///
    /// # Arguments
    /// - self reference
    ///
    /// # Returns
    /// - `Vec3` - the RGB color
    fn color(&self) -> Vec3;
}
