use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableList {
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
