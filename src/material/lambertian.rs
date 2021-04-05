use super::super::trace_ray;
use super::super::EPSILON;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::light::Light;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;
use rand::prelude::thread_rng as rng;
use rand::Rng;

/// Represent a Lambertial material with diffuse scattering
#[derive(Clone, Copy)]
pub struct Lambertian {
    /// Base albedo of the material
    pub albedo: Vec3,
}

/// Methods specific to Lambertian materials
impl Lambertian {
    /// Generate a vector on a Lambertian distribution
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// - `Vec3` - random direction
    fn random_direction() -> Vec3 {
        let theta = rng().gen::<f32>() * 2.0 * std::f32::consts::PI;
        let z = (rng().gen::<f32>() * 2.0) - 1.0;
        let r = (1.0 - z * z).sqrt();
        glm::vec3(r * theta.cos(), r * theta.sin(), z)
    }
}

/// Methods for the material trait
impl Material for Lambertian {
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
        _incoming_ray: &Ray,
        hit_record: &HitRecord,
        depth: u32,
    ) -> Vec3 {
        let mut scatter_direction = hit_record.normal() + Lambertian::random_direction();
        if scatter_direction.x.abs() < EPSILON
            && scatter_direction.y.abs() < EPSILON
            && scatter_direction.z.abs() < EPSILON
        {
            scatter_direction = hit_record.normal()
        }

        let scattered_ray = Ray::new(hit_record.hit_point, scatter_direction, Some(self.albedo));
        let scattered_color = trace_ray(&scattered_ray, world, lights, depth - 1);

        glm::matrix_comp_mult(&self.albedo, &scattered_color)
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
