use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::Hittable;
use crate::ray::Ray;

/// Represent a list of hittable objects
pub struct HittableList {
    /// Vector of hittables on the heap
    pub objects: Vec<Box<dyn Hittable>>,
    // Cached bounding box
    bounding_box: Option<AABB>,
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
            bounding_box: None,
        }
    }
    /// Add a hittable object to the list
    ///
    /// # Arguments
    /// - *mutable* self reference
    /// - `hittable: Box<dyn Hittable>` - new hittable to add to the collection
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
        let last_box = (&self.objects.last().unwrap()).bounding_box();
        self.bounding_box = if let Some(bbox) = self.bounding_box {
            if let Some(new_box) = last_box {
                Some(AABB::surrounding_box(&bbox, &new_box))
            } else {
                self.bounding_box
            }
        } else {
            last_box
        }
    }

    /// Create a HittableList from a vector of boxed hittables.
    ///
    /// # Arguments
    /// - vec, a vector of boxed hittables
    pub fn from_vec(vec: Vec<Box<dyn Hittable>>) -> HittableList {
        let mut list = HittableList::new();
        for object in vec.into_iter() {
            list.add(object);
        }
        list
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

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(&ray, min_distance, max_distance) {
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
        self.bounding_box
    }
}
