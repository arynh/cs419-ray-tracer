use glm::Vec3;

/// Represent a ray with an origin and direction.
#[derive(Clone, Copy)]
pub struct Ray {
    /// origin point of the ray
    pub origin: Vec3,
    /// direction along with the ray point, normalized
    pub direction: Vec3,
    /// color carried along this ray
    pub attenuation: Option<Vec3>,
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
        self.origin + t * self.direction
    }

    /// Create a new ray. The direction is normalized in this process.
    pub fn new(origin: Vec3, direction: Vec3, attenuation: Option<Vec3>) -> Ray {
        Ray {
            origin,
            direction: glm::normalize(&direction),
            attenuation,
        }
    }
}
