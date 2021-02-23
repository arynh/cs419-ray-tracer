use crate::camera::Camera;
use crate::ray::Ray;
use glm::Vec3;

/// Camera using the orthographic projection.
pub struct OrthographicCamera {
    /// Camera center in world coordinates
    pub origin: Vec3,
    /// Vector for the horizontal axis of the image plane
    pub horizontal: Vec3,
    /// Vector for the vertical axis of the image plane
    pub vertical: Vec3,
    /// Point at which the image plane starts
    pub lower_left_corner: Vec3,
    /// Private field to keep track of which direction the orthographic rays point
    orthogonal_direction: Vec3,
}

/// Methods for the orthographic camera
impl OrthographicCamera {
    /// Calculate the new camera parameters for the given configuration
    ///
    /// # Arguments
    /// - `position: Vec3` - new position of the camera
    /// - `lookat: Vec3` - new look-at point for the camera
    /// - `up_direction: Vec3` - new up direction
    /// - `vertical_fov: f32` - new vertical field of view in degrees
    /// - `aspect_ratio: f32` - new aspect ratio
    ///
    /// # Returns
    /// - tuple of
    ///     - `Vec3` - new position of the camera
    ///     - `Vec3` - new horizontal vector
    ///     - `Vec3` - new vertical vector
    ///     - `Vec3` - new lower left corner of the image plane
    fn calculate_camera_parameters(
        position: Vec3,
        lookat: Vec3,
        up_direction: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
    ) -> (Vec3, Vec3, Vec3, Vec3) {
        let h = (vertical_fov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let into_camera = glm::normalize(&(position - lookat));
        let horizontal_direction = glm::normalize(&glm::cross(&up_direction, &into_camera));
        let vertical_direction = glm::cross(&into_camera, &horizontal_direction);

        (
            position,
            viewport_width * horizontal_direction,
            viewport_height * vertical_direction,
            position
                - viewport_width * horizontal_direction / 2.0
                - viewport_height * vertical_direction / 2.0,
        )
    }

    /// Create a new camera struct from the given parameters.
    ///
    /// # Arguments
    /// - `position: Vec3` - position of the camera
    /// - `lookat: Vec3` - look-at point for the camera
    /// - `up_direction: Vec3` - up direction
    /// - `vertical_fov: f32` - vertical field of view in degrees
    /// - `aspect_ratio: f32` - aspect ratio
    ///
    /// # Returns
    /// - new `crate::camera::orthographic_camera::OrthographicCamera` struct
    pub fn new(
        position: Vec3,
        lookat: Vec3,
        up_direction: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
    ) -> OrthographicCamera {
        let (origin, horizontal, vertical, lower_left_corner) =
            OrthographicCamera::calculate_camera_parameters(
                position,
                lookat,
                up_direction,
                vertical_fov,
                aspect_ratio,
            );

        OrthographicCamera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            orthogonal_direction: lookat - origin,
        }
    }
}

/// Methods for the camera trait
impl Camera for OrthographicCamera {
    /// Get a ray to be traced from the scene to the camera.
    ///
    /// # Arguments
    /// - self reference
    /// - `u: f32` - horizontal parameter, from 0 to 1, on the image plane
    /// - `v: f32` - vertical parameter, from 0 to 1, on the image plane
    ///
    /// # Returns
    /// - the new ray to be traced
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.lower_left_corner + u * self.horizontal + v * self.vertical,
            direction: self.orthogonal_direction,
            attenuation: glm::vec3(0.0, 0.0, 0.0),
        }
    }

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
    ) {
        let (origin, horizontal, vertical, lower_left_corner) =
            OrthographicCamera::calculate_camera_parameters(
                position,
                lookat,
                up_direction,
                vertical_fov,
                aspect_ratio,
            );
        self.origin = origin;
        self.horizontal = horizontal;
        self.vertical = vertical;
        self.lower_left_corner = lower_left_corner;
        self.orthogonal_direction = lookat - origin;
    }
}
