use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

pub struct Triangle {
    pub vertices: (Vec3, Vec3, Vec3),
    pub material: Box<dyn Material>,
}

impl Hittable for Triangle {
    /// If a triangle will be hit by a ray in a certain range, return a
    /// hit record with the intersection information. Otherwise, return `None`.
    /// This code uses the Möller–Trumbore intersection algorithm, with code
    /// based on the wikipedia page's implementation.
    ///
    /// # Arguments
    /// - `ray` the ray to search for intersections along
    /// - `min_distance` the minimum distance of intersections along the ray
    /// - `max_distance` the maximum distance of intersections
    ///
    /// # Returns
    /// - Optional `HitRecord` if there was a hit, otherwise `None`.
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitRecord> {
        let edge_one = self.vertices.1 - self.vertices.0;
        let edge_two = self.vertices.2 - self.vertices.0;
        let h = glm::cross(&ray.direction, &edge_two);
        let a = glm::dot(&edge_one, &h);
        if a > -min_distance && a < min_distance {
            None // This ray is parallel to this triangle.
        } else {
            let f = 1.0 / a;
            let s = ray.origin - self.vertices.0;
            let u = f * glm::dot(&s, &h);
            if u < 0.0 || u > 1.0 {
                None
            } else {
                let q = glm::cross(&s, &edge_one);
                let v = f * glm::dot(&ray.direction, &q);
                if v < 0.0 || u + v > 1.0 {
                    None
                } else {
                    // compute location of intersection
                    let t = f * glm::dot(&edge_two, &q);
                    if t > min_distance && t < max_distance {
                        // intersection!
                        Some(HitRecord {
                            hit_point: ray.at(t),
                            ray: *ray,
                            distance: t,
                            outward_normal: glm::cross(&edge_one, &edge_two),
                            material: &(*self.material),
                        })
                    } else {
                        None
                    }
                }
            }
        }
    }
}
