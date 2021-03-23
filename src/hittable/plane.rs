use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::Hittable;
use crate::material::MaterialType;
use crate::ray::Ray;
use glm::Vec3;

/// Represent a plane in space
pub struct Plane {
    /// center point of the plane
    pub center: Vec3,
    /// normal vector of the plane from the center point
    pub normal: Vec3,
    /// material of the plane
    pub material: MaterialType,
}

/// Methods for the hittable trait
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
                    material: Some(&self.material),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// A plane is infinite, so it has no bounding box.
    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}
