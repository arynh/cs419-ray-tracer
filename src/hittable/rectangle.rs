use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::triangle::Triangle;
use crate::hittable::triangle::TriangleList;
use crate::hittable::Hittable;
use crate::material::MaterialType;
use crate::ray::Ray;
use glm::Vec3;

/// Represent a rectangular object as two triangles.
pub struct Rectangle {
    pub material: MaterialType,
    triangles: TriangleList,
}

/// Methods for Rectangle
impl Rectangle {
    /// Create a new rectangle.
    ///
    /// # Arguments
    /// - `points: [Vec3; 4]` - Four corners of the rectangle in counter-
    ///   clockwise order.
    /// - `material: MaterialType` - material of the rectangle
    pub fn new(points: [Vec3; 4], material: MaterialType) -> Rectangle {
        let triangle_one = Triangle::new([points[0], points[1], points[2]], material);
        let triangle_two = Triangle::new([points[2], points[3], points[0]], material);

        Rectangle {
            material,
            triangles: TriangleList::new(vec![triangle_one, triangle_two]),
        }
    }
}

/// Methods from the hittable trait
impl Hittable for Rectangle {
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
        self.triangles.hit(ray, min_distance, max_distance)
    }

    /// Return the rectangle's bounding box.
    ///
    /// # Arguments
    /// - just the self reference
    ///
    /// # Returns
    /// - Optional 'AABB' which encloses the hittable.
    fn bounding_box(&self) -> Option<AABB> {
        self.triangles.bounding_box()
    }
}
