use super::DIFFUSE_WEIGHT;
use super::EPSILON;
use super::MAX_HIT_DISTANCE;
use super::SPECULAR_COEFFICIENT;
use super::SPECULAR_WEIGHT;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

/// Represent a point light source
pub struct Light {
    /// position of the light source in world coordinates
    pub position: Vec3,
    /// relative strength of the light source
    pub weight: f32,
}

/// Point light implementation
impl Light {
    /// Shade the given hit point according to the Blinn-Phong model. The
    /// global component is done by the ray tracer, so this method just
    /// calculates the component of shading from diffuse and specular
    /// reflections.
    pub fn shade(&self, hit: &HitRecord, world: &dyn Hittable) -> Vec3 {
        // calculate ray from hit point to light source
        let point_to_light_vector = self.position - hit.hit_point;
        let point_to_light = Ray::new(hit.hit_point, point_to_light_vector, None);
        // cast a new ray to the light to see if it hits anything
        if let Some(_shadow_hit) = &world.hit(&point_to_light, EPSILON, MAX_HIT_DISTANCE) {
            // shadow => no diffuse or specular components
            glm::vec3(0.0, 0.0, 0.0)
        } else {
            let material_color = &hit.material.unwrap().color();
            let normal_vector = &hit.normal();
            let diffuse_light_weighting =
                glm::normalize_dot(&point_to_light.direction, &normal_vector).max(0.0);
            let halfway_vector = point_to_light.direction - hit.ray.direction;
            let specular_light_weighting = glm::normalize_dot(&normal_vector, &halfway_vector);
            material_color * diffuse_light_weighting * DIFFUSE_WEIGHT
                + glm::vec3(1.0, 1.0, 1.0)
                    * specular_light_weighting.powf(SPECULAR_COEFFICIENT)
                    * SPECULAR_WEIGHT
        }
    }
}
