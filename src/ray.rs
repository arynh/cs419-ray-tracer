use glm::Vec3;

/// Represent a ray with an origin and direction.
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub attenuation: Vec3,
}

impl Ray {
    /// Give a point along the ray according to parameter `t`.
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
