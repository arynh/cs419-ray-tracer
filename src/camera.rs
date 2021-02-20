pub mod orthographic_camera;
pub mod perspective_camera;

use crate::ray::Ray;

pub trait Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray;
}
