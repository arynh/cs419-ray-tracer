use crate::camera::Camera;
use crate::ray::Ray;
use glm::Vec3;

pub struct OrthographicCamera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    look_at: Vec3,
}

impl OrthographicCamera {
    pub fn new_default_orthographic() -> OrthographicCamera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = -2.0;
        let origin = glm::vec3(0.0, 0.0, 0.0);
        let horizontal = glm::vec3(viewport_width, 0.0, 0.0);
        let vertical = glm::vec3(0.0, viewport_height, 0.0);
        let look_at = glm::vec3(0.0, 0.0, -1.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - glm::vec3(0.0, 0.0, focal_length);

        OrthographicCamera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            look_at: look_at,
        }
    }
}

impl Camera for OrthographicCamera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
            direction: self.look_at,
            attenuation: glm::vec3(0.0, 0.0, 0.0),
        }
    }
}
