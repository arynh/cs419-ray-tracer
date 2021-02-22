use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

/// Represent a list of hittable objects
pub struct HittableList {
    /// Vector of hittables on the heap
    objects: Vec<Box<dyn Hittable>>,
}

/// Methods for hittable lists
impl HittableList {
    /// Construct a new hittable list
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// - `HittableList` - the empty list
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    /// Add a hittable object to the list
    ///
    /// # Arguments
    /// - *mutable* self reference
    /// - `hittable: Box<dyn Hittable>` - new hittable to add to the collection
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

/// Methods from the hittable trait
impl Hittable for HittableList {
    /// If an object will be hit by a ray in a certain range, return a
    /// hit record with the intersection information. Otherwise, return `None`.
    ///
    /// For a hittable list, _all_ objects in the scene are tested along the
    /// ray, and the closest collision is returned.
    ///
    /// # Arguments
    /// - `ray` the ray to search for intersections along
    /// - `min_distance` the minimum distance of intersections along the ray
    /// - `max_distance` the maximum distance of intersections
    ///
    /// # Returns
    /// - Optional `HitRecord` if there was a hit, otherwise `None`.
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitRecord> {
        let mut current_min = std::f32::INFINITY;
        let mut closest_hit: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, min_distance, max_distance) {
                if hit.distance < current_min {
                    current_min = hit.distance;
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }
}
