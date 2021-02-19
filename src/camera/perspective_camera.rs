use crate::camera::Camera;
use crate::ray::Ray;
use glm::Vec3;

pub struct PerspectiveCamera {
    aspect_ratio: f32,
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,

    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl PerspectiveCamera {
    pub fn new_default_perspective() -> PerspectiveCamera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = glm::vec3(0.0, 0.0, 0.0);
        let horizontal = glm::vec3(viewport_width, 0.0, 0.0);
        let vertical = glm::vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - glm::vec3(0.0, 0.0, focal_length);

        PerspectiveCamera {
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_length: focal_length,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
            attenuation: glm::vec3(0.0, 0.0, 0.0),
        }
    }
}
