use super::super::trace_ray;
use crate::color;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::light::Light;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

/// Represent a metal material with reflection
#[derive(Clone, Copy)]
pub struct Metal {
    /// Base albedo of the material
    pub albedo: Vec3,
}

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
    fn shade<T: Hittable>(
        &self,
        world: &T,
        lights: &[Light],
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        depth: u32,
    ) -> Vec3 {
        let reflected_direction = glm::reflect_vec(&incoming_ray.direction, &hit_record.normal());
        if glm::dot(&reflected_direction, &hit_record.normal()) > 0.0 {
            let reflected_ray =
                Ray::new(hit_record.hit_point, reflected_direction, Some(self.albedo));
            trace_ray(&reflected_ray, world, lights, depth - 1)
        } else {
            color::color(0, 0, 0)
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
