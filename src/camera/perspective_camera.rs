use crate::camera::Camera;
use crate::ray::Ray;
use glm::Vec3;

pub struct PerspectiveCamera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl PerspectiveCamera {
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
                - viewport_height * vertical_direction / 2.0
                - into_camera,
        )
    }

    pub fn new(
        position: Vec3,
        lookat: Vec3,
        up_direction: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
    ) -> PerspectiveCamera {
        let (origin, horizontal, vertical, lower_left_corner) =
            PerspectiveCamera::calculate_camera_parameters(
                position,
                lookat,
                up_direction,
                vertical_fov,
                aspect_ratio,
            );

        PerspectiveCamera {
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

    fn move_camera(
        &mut self,
        position: Vec3,
        lookat: Vec3,
        up_direction: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
    ) {
        let (origin, horizontal, vertical, lower_left_corner) =
            PerspectiveCamera::calculate_camera_parameters(
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
    }
}
