use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::triangle::Triangle;
use crate::hittable::triangle::TriangleList;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub enum BVHNode {
    BVH(Box<BVH>),
    HittableList(Box<TriangleList>),
}

pub struct BVH {
    /// Left subtree
    pub left: BVHNode,
    /// Right subtree
    pub right: BVHNode,
    /// Bounding box of this (sub)tree
    pub bounding_box: AABB,
}

impl Hittable for BVHNode {
    /// Forward the hit call to the appropriate method for each type of BVH node.
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
            BVHNode::BVH(ref node) => node.hit(&ray, min_distance, max_distance),
            BVHNode::HittableList(ref node) => node.hit(&ray, min_distance, max_distance),
        }
    }

    /// Return the bounding box for this node.
    fn bounding_box(&self) -> Option<AABB> {
        match *self {
            BVHNode::BVH(ref node) => node.bounding_box(),
            BVHNode::HittableList(ref node) => node.bounding_box(),
        }
    }
}

impl Hittable for BVH {
    /// If an object in this BVH will be hit by a ray in a certain range, return a
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
        if let Some(_hit) = &self.bounding_box.hit(&ray, min_distance, max_distance) {
            if let Some(hit_left) = self.left.hit(&ray, min_distance, max_distance) {
                if let Some(hit_right) = self.right.hit(&ray, min_distance, hit_left.distance) {
                    Some(hit_right)
                } else {
                    Some(hit_left)
                }
            } else {
                self.right.hit(&ray, min_distance, max_distance)
            }
        } else {
            None
        }
    }

    /// Return the bounding box for this BVH.
    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

impl BVH {
    pub fn build(objects: Vec<Triangle>, max_at_leaf: usize) -> BVH {
        // compute all bounding boxes and centroids
        let mut bounding_boxes: Vec<AABB> = Vec::new();
        let mut centroids: Vec<glm::Vec3> = Vec::new();
        for object in objects.iter() {
            if let Some(bbox) = object.bounding_box() {
                bounding_boxes.push(bbox);
                centroids.push(bbox.centroid());
            }
        }

        // compute the bounding box of this BVH (sub)tree
        let bounding_box = bounding_boxes
            .iter()
            .fold(bounding_boxes[0], |expanding, next| {
                AABB::surrounding_box(&expanding, &*next)
            });

        // find the axis with the greatest spread
        let mut maximal_spread: f32 = 0.0;
        let mut split_axis: usize = 0;
        for axis in 0..3 {
            let spread = centroids
                .iter()
                .fold(f32::NEG_INFINITY, |max, c| max.max(c[axis]))
                - centroids
                    .iter()
                    .fold(f32::INFINITY, |min, c| min.min(c[axis]));
            if spread > maximal_spread {
                maximal_spread = spread;
                split_axis = axis;
            }
        }

        // find the midpoint of centroids along the most spread axis
        let projected_centroids: Vec<f32> = centroids.iter().map(|c| c[split_axis]).collect();
        let midpoint = projected_centroids.iter().sum::<f32>() / (projected_centroids.len() as f32);

        // partition the objects to the 'left' and 'right' of the midpoint
        let mut lefts: Vec<Triangle> = Vec::new();
        let mut rights: Vec<Triangle> = Vec::new();
        for obj in objects.into_iter() {
            let mut which = true;
            if let Some(bbox) = obj.bounding_box() {
                which = bbox.centroid()[split_axis] < midpoint;
            }
            if which {
                lefts.push(obj);
            } else {
                rights.push(obj);
            }
        }

        let left: BVHNode;
        if lefts.len() > max_at_leaf {
            left = BVHNode::BVH(Box::new(BVH::build(lefts, max_at_leaf)));
        } else {
            left = BVHNode::HittableList(Box::new(TriangleList::new(lefts)));
        }
        let right: BVHNode;
        if rights.len() > max_at_leaf {
            right = BVHNode::BVH(Box::new(BVH::build(rights, max_at_leaf)));
        } else {
            right = BVHNode::HittableList(Box::new(TriangleList::new(rights)));
        }

        BVH {
            left,
            right,
            bounding_box,
        }
    }
}
