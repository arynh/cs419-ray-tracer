use super::super::trace_ray;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::light::Light;
use crate::material::Material;
use crate::ray::Ray;
use crate::scenes::Sky;
use glm::Vec3;

/// Represent a transparent material with reflection, refraction, and absorption
#[derive(Clone, Copy)]
pub struct Transparent {
    /// base color of the material
    pub albedo: Vec3,
    /// proportion of light reflected
    pub reflectance: f32,
    /// proportion of light refracted
    pub transmittance: f32,
    /// Refractive index specific to this material
    pub refractive_index: f32,
}

/// Methods for the material trait
impl Material for Transparent {
    /// Shade a hit to this material.
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
        sky: &Sky,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        depth: u32,
    ) -> Vec3 {
        // compute reflected light
        let reflected_direction = glm::reflect_vec(&incoming_ray.direction, &hit_record.normal());
        let reflected_ray = Ray::new(hit_record.hit_point, reflected_direction, Some(self.albedo));
        let reflectance = self.reflectance * self.albedo
            / glm::dot(&hit_record.outward_normal, &reflected_direction);
        let reflected_color = trace_ray(&reflected_ray, world, lights, sky, depth - 1);

        // check for total internal reflection
        if Transparent::total_internal_reflection(hit_record, incoming_ray, self.refractive_index) {
            return reflected_color;
        }

        // compute refracted light
        let mut normal = hit_record.outward_normal;
        let mut eta = self.refractive_index;
        let incoming_direction = -incoming_ray.direction;
        let mut cos_theta_i = glm::dot(&normal, &incoming_direction);

        if cos_theta_i < 0.0 {
            cos_theta_i = -cos_theta_i;
            normal = -normal;
            eta = 1.0 / eta;
        }

        let cos_theta_2 = (1.0 - (1.0 - cos_theta_i * cos_theta_i) / (eta * eta)).sqrt();
        let transmitted_direction =
            -incoming_direction / eta - (cos_theta_2 - cos_theta_i / eta) * normal;
        let transmitted_ray = Ray::new(
            hit_record.hit_point,
            transmitted_direction,
            Some(self.albedo),
        );
        let transmittance = self.transmittance / (eta * eta) * self.albedo
            / glm::dot(&hit_record.outward_normal, &transmitted_direction).abs();
        let transmitted_color = trace_ray(&transmitted_ray, world, lights, sky, depth - 1);

        glm::matrix_comp_mult(&reflectance, &reflected_color)
            * glm::dot(&hit_record.outward_normal, &reflected_direction).abs()
            + glm::matrix_comp_mult(&transmittance, &transmitted_color)
                * glm::dot(&hit_record.outward_normal, &transmitted_direction).abs()
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

/// Methods specific to transparent materials
impl Transparent {
    /// Test for total internal reflection.
    fn total_internal_reflection(
        hit_record: &HitRecord,
        incoming_ray: &Ray,
        refractive_index: f32,
    ) -> bool {
        let cos_theta_i = glm::dot(&hit_record.outward_normal, &-incoming_ray.direction);

        let eta = if cos_theta_i < 0.0 {
            1.0 / refractive_index
        } else {
            refractive_index
        };

        1.0 - (1.0 - cos_theta_i * cos_theta_i) / (eta * eta) < 0.0
    }
}
