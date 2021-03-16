use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::Hittable;
use crate::ray::Ray;

/// Represent a list of hittable objects
pub struct HittableList {
    /// Vector of hittables on the heap
    pub objects: Vec<Box<dyn Hittable>>,
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

    /// Create a HittableList from a vector of boxed hittables.
    ///
    /// # Arguments
    /// - vec, a vector of boxed hittables
    pub fn from_vec(vec: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { objects: vec }
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

    /// Find the box which bounds all objects in the hittable list.
    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box = if let Some(bbox) = self.objects[0].bounding_box() {
            bbox
        } else {
            return None;
        };

        let mut expanding_box: AABB;
        let mut first_box = true;

        for object in self.objects.iter() {
            if let Some(bbox) = object.bounding_box() {
                expanding_box = bbox;
                output_box = if first_box {
                    expanding_box
                } else {
                    AABB::surrounding_box(output_box, expanding_box)
                };
                first_box = false;
            } else {
                return None;
            }
        }
        Some(output_box)
    }
}
