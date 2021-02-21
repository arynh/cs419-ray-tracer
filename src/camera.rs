pub mod orthographic_camera;
pub mod perspective_camera;

use crate::ray::Ray;
use glm::Vec3;

pub trait Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray;
    fn move_camera(
        &mut self,
        position: Vec3,
        lookat: Vec3,
        up_direction: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
    );
}
