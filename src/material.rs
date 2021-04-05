pub mod lambertian;
pub mod metal;
pub mod transparent;

use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::light::Light;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::transparent::Transparent;
use crate::ray::Ray;
use glm::Vec3;

/// Material trait
pub trait Material {
    /// Determine where the next ray goes after a hit depending on this
    /// material.
    ///
    /// # Arguments
    /// - self reference
    /// - `incoming_ray` - ray which has just hit this material
    /// - `hit_record` - specification of the hit which just ocurred
    ///
    /// # Returns
    /// - optional `Ray`, or none if the ray was absorbed
    fn shade<T: Hittable>(
        &self,
        world: &T,
        lights: &[Light],
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        depth: u32,
    ) -> Vec3;

    /// Retrieve the base color of the material.
    ///
    /// # Arguments
    /// - self reference
    ///
    /// # Returns
    /// - `Vec3` - the RGB color
    fn color(&self) -> Vec3;
}

#[derive(Clone, Copy)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Transparent(Transparent),
}

impl Material for MaterialType {
    /// Determine where the next ray goes after a hit depending on this
    /// material.
    ///
    /// # Arguments
    /// - self reference
    /// - `incoming_ray` - ray which has just hit this material
    /// - `hit_record` - specification of the hit which just ocurred
    ///
    /// # Returns
    /// - optional `Ray`, or none if the ray was absorbed
    fn shade<T: Hittable>(
        &self,
        world: &T,
        lights: &[Light],
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        depth: u32,
    ) -> Vec3 {
        match *self {
            MaterialType::Lambertian(ref material) => {
                material.shade(world, lights, incoming_ray, hit_record, depth)
            }
            MaterialType::Metal(ref material) => {
                material.shade(world, lights, incoming_ray, hit_record, depth)
            }
            MaterialType::Transparent(ref material) => {
                material.shade(world, lights, incoming_ray, hit_record, depth)
            }
        }
    }

    /// Retrieve the base color of the material.
    ///
    /// # Arguments
    /// - self reference
    ///
    /// # Returns
    /// - `Vec3` - the RGB color
    fn color(&self) -> Vec3 {
        match *self {
            MaterialType::Lambertian(ref material) => material.color(),
            MaterialType::Metal(ref material) => material.color(),
            MaterialType::Transparent(ref material) => material.color(),
        }
    }
}
