use super::DIFFUSE_WEIGHT;
use super::EPSILON;
use super::MAX_HIT_DISTANCE;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
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
    /// Shade the given hit point according to the Blinn-Phong model, but with
    /// no specular component of lighting. The global component is done by the
    /// ray tracer, so this method just calculates the component of shading
    /// from diffuse reflections.
    pub fn shade_diffuse(&self, hit: &HitRecord, world: &HittableList) -> Vec3 {
        // calculate ray from hit point to light source
        let point_to_light = Ray {
            origin: hit.hit_point,
            direction: self.position - hit.hit_point,
            attenuation: glm::vec3(0.0, 0.0, 0.0),
        };
        // cast a new ray to the light to see if it hits anything
        if let Some(_shadow_hit) = world.hit(&point_to_light, EPSILON, MAX_HIT_DISTANCE) {
            // shadow => no diffuse component
            glm::vec3(0.0, 0.0, 0.0)
        } else {
            let material_color = hit.material.color();
            let normal_vector = hit.normal();
            let diffuse_light_weighting =
                glm::normalize_dot(&point_to_light.direction, &normal_vector).max(0.0);
            material_color * diffuse_light_weighting * DIFFUSE_WEIGHT
        }
    }
}
