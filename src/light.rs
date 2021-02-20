use super::DIFFUSE_WEIGHT;
use super::EPSILON;
use super::MAX_HIT_DISTANCE;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use glm::Vec3;

pub struct Light {
    pub position: Vec3,
    pub weight: f32,
}

impl Light {
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
