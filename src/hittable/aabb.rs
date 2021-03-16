use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use glm::Vec3;

/// Model an axis-aligned bounding box
#[derive(Clone, Copy)]
pub struct AABB {
    /// Minimal point of the bounded space
    pub minimum_point: Vec3,
    /// Maximal point of the bounded space
    pub maximum_point: Vec3,
}

impl Hittable for AABB {
    /// If an AABB will be hit by a ray in a certain range, return a
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
        for dimension in 0..3 {
            let t0: f32 = ((self.minimum_point[dimension] - ray.origin[dimension])
                / ray.direction[dimension])
                .min(
                    (self.maximum_point[dimension] - ray.origin[dimension])
                        / ray.direction[dimension],
                );
            let t1: f32 = ((self.minimum_point[dimension] - ray.origin[dimension])
                / ray.direction[dimension])
                .max(
                    (self.maximum_point[dimension] - ray.origin[dimension])
                        / ray.direction[dimension],
                );
            let t_min = t0.max(min_distance);
            let t_max = t1.min(max_distance);
            if t_max <= t_min {
                return None;
            }
        }
        Some(HitRecord {
            hit_point: glm::vec3(0.0, 0.0, 0.0),
            ray: *ray,
            distance: 0.0,
            outward_normal: glm::vec3(0.0, 0.0, 0.0),
            material: None,
        })
    }

    /// The bounding box of a bounding box is itself.
    fn bounding_box(&self) -> Option<AABB> {
        Some(*self)
    }
}

impl AABB {
    /// Produce a new box which surrounds both of the given boxes.
    ///
    /// # Arguments
    /// - `first` the first box
    /// - `second` another box
    ///
    /// # Returns
    /// - A new box which bounds all of the space of the given boxes.
    pub fn surrounding_box(first: AABB, second: AABB) -> AABB {
        AABB {
            minimum_point: glm::vec3(
                first.minimum_point.x.min(second.minimum_point.x),
                first.minimum_point.y.min(second.minimum_point.y),
                first.minimum_point.z.min(second.minimum_point.z),
            ),
            maximum_point: glm::vec3(
                first.maximum_point.x.max(second.maximum_point.x),
                first.maximum_point.y.max(second.maximum_point.y),
                first.maximum_point.z.max(second.maximum_point.z),
            ),
        }
    }

    /// Find the centroid of this AABB. This is the point between the minimum
    /// and maximum extent of the bounding volume.
    ///
    /// # Arguments
    /// - self reference
    ///
    /// # Returns
    /// - a `Vec3` containing the centroid of the AABB
    pub fn centroid(&self) -> Vec3 {
        (self.minimum_point + self.maximum_point) / 2.0
    }
}
