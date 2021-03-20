use glm::Vec3;

/// Represent a ray with an origin and direction.
#[derive(Clone, Copy)]
pub struct Ray {
    /// origin point of the ray
    pub origin: Vec3,
    /// direction along with the ray point, not guaranteed to be normalized
    pub direction: Vec3,
}

/// Methods for the ray struct
impl Ray {
    /// Give a point along the ray according to distance parameter `t`.
    ///
    /// # Arguments
    /// - self reference
    /// - `t: f32` - the distance parameter
    ///
    /// # Returns
    /// - `Vec3` - Point at t distance along the ray
    pub fn at(&self, t: f32) -> Vec3 {
        &self.origin + t * glm::normalize(&self.direction)
    }
}
