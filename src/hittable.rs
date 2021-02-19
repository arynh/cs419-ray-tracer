pub mod plane;
pub mod sphere;
pub mod triangle;

use crate::hit_record::HitRecord;
use crate::ray::Ray;

/// A trait of objects that are "hittable," meaning that rays cast through
/// the schene can interact with the object.
pub trait Hittable {
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
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitRecord>;
}
