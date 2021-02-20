use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
}

impl Hittable for Plane {
    /// If a plane will be hit by a ray in a certain range, return a
    /// hit record with the intersection information. Otherwise, return `None`.
    ///
    /// # Arguments
    /// - `ray` the ray to search for intersections along
    /// - `min_distance` the minimum distance of intersections along the ray
    /// - `max_distance` the maximum distance of intersections
    ///
    /// # Returns
    /// - Optional `HitRecord` if there was a hit, otherwise `None`.
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitRecord> {
        let angle = glm::dot(&self.normal, &ray.direction);
        if angle.abs() > min_distance {
            let t = glm::dot(&(self.center - ray.origin), &self.normal) / angle;
            if t > min_distance && t < max_distance {
                Some(HitRecord {
                    hit_point: ray.at(t),
                    ray: *ray,
                    distance: t,
                    outward_normal: self.normal,
                    material: &(*self.material),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
