pub mod aabb;
pub mod bvh;
pub mod hittable_list;
pub mod mesh;
pub mod plane;
pub mod rectangle;
pub mod sphere;
pub mod triangle;

use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::mesh::Mesh;
use crate::hittable::plane::Plane;
use crate::hittable::rectangle::Rectangle;
use crate::hittable::sphere::Sphere;
use crate::hittable::triangle::Triangle;
use crate::hittable::triangle::TriangleList;
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

    /// Finite objects fit into bounding boxes. If this is the case, they
    /// can provide their bounding box.
    ///
    /// # Arguments
    /// - just the self reference
    ///
    /// # Returns
    /// - Optional 'AABB' which encloses the hittable.
    fn bounding_box(&self) -> Option<AABB>;
}

/// Enumerate all possible hittables here. These are the only hittables, so
/// dynamic dispatch is avoided.
pub enum HittableItem {
    Mesh(Mesh),
    Plane(Plane),
    Rectangle(Rectangle),
    Sphere(Sphere),
    Triangle(Triangle),
    TriangleList(TriangleList),
}

/// Forward Hittable methods to the correct child of the enum.
impl Hittable for HittableItem {
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
        match *self {
            HittableItem::Mesh(ref hittable) => hittable.hit(ray, min_distance, max_distance),
            HittableItem::Plane(ref hittable) => hittable.hit(ray, min_distance, max_distance),
            HittableItem::Rectangle(ref hittable) => hittable.hit(ray, min_distance, max_distance),
            HittableItem::Sphere(ref hittable) => hittable.hit(ray, min_distance, max_distance),
            HittableItem::Triangle(ref hittable) => hittable.hit(ray, min_distance, max_distance),
            HittableItem::TriangleList(ref hittable) => {
                hittable.hit(ray, min_distance, max_distance)
            }
        }
    }

    /// Finite objects fit into bounding boxes. If this is the case, they
    /// can provide their bounding box.
    ///
    /// # Arguments
    /// - just the self reference
    ///
    /// # Returns
    /// - Optional 'AABB' which encloses the hittable.
    fn bounding_box(&self) -> Option<AABB> {
        match *self {
            HittableItem::Mesh(ref hittable) => hittable.bounding_box(),
            HittableItem::Plane(ref hittable) => hittable.bounding_box(),
            HittableItem::Rectangle(ref hittable) => hittable.bounding_box(),
            HittableItem::Sphere(ref hittable) => hittable.bounding_box(),
            HittableItem::Triangle(ref hittable) => hittable.bounding_box(),
            HittableItem::TriangleList(ref hittable) => hittable.bounding_box(),
        }
    }
}
