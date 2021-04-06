use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::light::Light;
use crate::material::Material;
use crate::ray::Ray;
use crate::scenes::Sky;
use glm::Vec3;

#[derive(Clone, Copy)]
pub struct DiffuseLight {
    pub color: Vec3,
}

impl Material for DiffuseLight {
    /// To shade a light, just return the color it emits
    fn shade<T: Hittable>(
        &self,
        _world: &T,
        _lights: &[Light],
        _sky: &Sky,
        _incoming_ray: &Ray,
        _hit_record: &HitRecord,
        _depth: u32,
    ) -> Vec3 {
        self.color
    }

    /// Retrieve the base color of the material.
    fn color(&self) -> Vec3 {
        self.color
    }
}
