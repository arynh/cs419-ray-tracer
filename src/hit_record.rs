use crate::material::Material;
use crate::ray::Ray;
use glm::Vec3;

/// Record a hit point for a ray and the normal at that hit point.
pub struct HitRecord<'a> {
    /// Location in space of the ray intersection
    pub hit_point: Vec3,
    /// The ray which intersected
    pub ray: Ray,
    /// Position along the ray (the t value)
    pub distance: f32,
    /// Normal vector of the geometry hit by the ray
    pub outward_normal: Vec3,
    /// Material hit
    pub material: Option<&'a dyn Material>,
}

impl<'a> HitRecord<'a> {
    /// Is this face visible to the camera?
    ///
    /// # Arguments
    /// - self reference
    ///
    /// # Returns
    /// - boolean value indicating if the face is visible to the camera
    pub fn is_front_face(&self) -> bool {
        glm::dot(&self.ray.direction, &self.outward_normal) < 0.0
    }

    /// Normal vector at this intersection point.
    ///
    /// # Arguments
    /// - self reference
    ///
    /// # Returns
    /// - the normal vector corrected for the position of the camera
    pub fn normal(&self) -> Vec3 {
        if self.is_front_face() {
            self.outward_normal
        } else {
            -self.outward_normal
        }
    }
}
