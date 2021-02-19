use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

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
                return Some(HitRecord {
                    hit_point: ray.at(x),
                    ray: *ray,
                    distance: x,
                    outward_normal: (ray.at(x) - self.center) / self.radius,
                    material: &(*self.material),
                });
            } else {
                x = (-half_b + root) / a;
                if x < max_distance && x > min_distance {
                    return Some(HitRecord {
                        hit_point: ray.at(x),
                        ray: *ray,
                        distance: x,
                        outward_normal: (ray.at(x) - self.center) / self.radius,
                        material: &(*self.material),
                    });
                }
            }
        }
        None
    }
}
