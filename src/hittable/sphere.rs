use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

/// Represent a sphere in space
pub struct Sphere {
    /// center point of the sphere
    pub center: Vec3,
    /// radius of the sphere
    pub radius: f32,
    /// material to use for the sphere
    pub material: Box<dyn Material>,
}

/// Methods from the hittable trait
impl Hittable for Sphere {
    /// If an object will be hit by a ray in a certain range, return a
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
        // calculate the discriminant
        let oc = ray.origin - self.center;
        let a = glm::dot(&ray.direction, &ray.direction);
        let half_b = glm::dot(&oc, &ray.direction);
        let c = glm::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut x = (-half_b - root) / a;
            if x < max_distance && x > min_distance {
                Some(HitRecord {
                    hit_point: ray.at(x),
                    ray: *ray,
                    distance: x,
                    outward_normal: (ray.at(x) - self.center) / self.radius,
                    material: Some(&(*self.material)),
                })
            } else {
                x = (-half_b + root) / a;
                if x < max_distance && x > min_distance {
                    Some(HitRecord {
                        hit_point: ray.at(x),
                        ray: *ray,
                        distance: x,
                        outward_normal: (ray.at(x) - self.center) / self.radius,
                        material: Some(&(*self.material)),
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    /// Calculate the bounding box for this sphere.
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            minimum_point: self.center - glm::vec3(self.radius, self.radius, self.radius),
            maximum_point: self.center + glm::vec3(self.radius, self.radius, self.radius),
        })
    }
}
