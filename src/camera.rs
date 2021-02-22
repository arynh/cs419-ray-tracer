pub mod orthographic_camera;
pub mod perspective_camera;

use crate::ray::Ray;
use glm::Vec3;

/// Represent any camera in the scene. Details aside, any camera must be able
/// to return a ray to be sampled at a specific point on the imaging plane, and
/// also allow its properties to be changed so it may be animated.
pub trait Camera {
    /// Get a ray to be traced from the scene to the camera.
    ///
    /// # Arguments
    /// - self reference
    /// - `u: f32` - horizontal parameter, from 0 to 1, on the image plane
    /// - `v: f32` - vertical parameter, from 0 to 1, on the image plane
    ///
    /// # Returns
    /// - the new ray to be traced
    fn get_ray(&self, u: f32, v: f32) -> Ray;

    /// Move the camera to a new location and change the fov or aspect ratio of
    /// the camera.
    ///
    /// # Arguments
    /// - self reference
    /// - `position: Vec3` - new position of the camera
    /// - `lookat: Vec3` - new look-at point for the camera
    /// - `up_direction: Vec3` - new up direction
    /// - `vertical_fov: f32` - new vertical field of view in degrees
    /// - `aspect_ratio: f32` - new aspect ratio
    fn move_camera(
        &mut self,
        position: Vec3,
        lookat: Vec3,
        up_direction: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
    );
}
