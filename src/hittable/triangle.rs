use super::super::EPSILON;
use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

/// Represent a triangle in space
pub struct Triangle {
    /// Vertices of the triangle
    pub vertices: [Vec3; 3],
    /// First two edges of the triangle
    pub edges: [Vec3; 2],
    /// Normals at each vertex
    pub vertex_normals: [Vec3; 3],
    /// Material of the triangle
    pub material: Box<dyn Material>,
}

/// Methods from the hittable trait
impl Hittable for Triangle {
    /// If a triangle will be hit by a ray in a certain range, return a
    /// hit record with the intersection information. Otherwise, return `None`.
    /// This code uses the Möller–Trumbore intersection algorithm, with code
    /// based on the wikipedia page's implementation.
    ///
    /// See `README.md` for the reference to Möller-Trumbore.
    ///
    /// # Arguments
    /// - `ray` the ray to search for intersections along
    /// - `min_distance` the minimum distance of intersections along the ray
    /// - `max_distance` the maximum distance of intersections
    ///
    /// # Returns
    /// - Optional `HitRecord` if there was a hit, otherwise `None`.
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitRecord> {
        let edge_one = &self.edges[0];
        let edge_two = &self.edges[1];
        let h = glm::cross(&ray.direction, &edge_two);
        let a = glm::dot(&edge_one, &h);
        if a > -EPSILON && a < EPSILON {
            None // This ray is parallel to this triangle.
        } else {
            let f = 1.0 / a;
            let s = ray.origin - self.vertices[0];
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
                            outward_normal: self.interpolate_normal(ray.at(t)),
                            material: Some(&(*self.material)),
                        })
                    } else {
                        None
                    }
                }
            }
        }
    }

    /// Compute the bounding box of this triangle.
    fn bounding_box(&self) -> Option<AABB> {
        let mut min_point = glm::vec3(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max_point = glm::vec3(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        for point in self.vertices.iter() {
            for dimension in 0..3 {
                if point[dimension] < min_point[dimension] {
                    min_point[dimension] = point[dimension];
                }
                if point[dimension] > max_point[dimension] {
                    max_point[dimension] = point[dimension];
                }
            }
        }
        Some(AABB {
            minimum_point: min_point,
            maximum_point: max_point,
        })
    }
}

impl Triangle {
    /// Given a hit location, interpolate the vertex normals to get the normal
    /// at the hit point.
    ///
    /// This method is modelled after this approach:
    /// https://gamedev.stackexchange.com/a/23745
    ///
    /// # Arguments
    /// - self reference
    /// - `hit_location` - a `Vec3` representing a point on the triangle.
    fn interpolate_normal(&self, hit_location: Vec3) -> Vec3 {
        let edge_one = &self.edges[0];
        let edge_two = &self.edges[1];
        let point_to_hit = hit_location - self.vertices[0];

        let d00 = glm::dot(&edge_one, &edge_one);
        let d01 = glm::dot(&edge_one, &edge_two);
        let d11 = glm::dot(&edge_two, &edge_two);
        let d20 = glm::dot(&point_to_hit, &edge_one);
        let d21 = glm::dot(&point_to_hit, &edge_two);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        glm::normalize(
            &(u * self.vertex_normals[0] + v * self.vertex_normals[1] + w * self.vertex_normals[2]),
        )
    }
}
