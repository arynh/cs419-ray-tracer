use glm::Vec3;

/// Convert from vector to gamma adjusted and clamped RGB values.
///
/// # Arguments
/// - `vec: &Vec3` - Vec3 to convert to a RGB pixel
pub fn vec3_to_rgb(vec: &Vec3, samples_level: usize) -> image::Rgb<u8> {
    // scale by samples per pixel
    let scaled = vec / (samples_level * samples_level) as f32;
    // gamma correction
    let g = 1.0 / 2.2;
    let adjusted = glm::pow(&scaled, &glm::vec3(g, g, g));
    // clamp and convert to u8 RGB
    let clamped = glm::clamp(&adjusted, 0.0, 1.0);
    let converted = clamped * 255.0;
    image::Rgb([converted.x as u8, converted.y as u8, converted.z as u8])
}

/// Utility to convert from 8 bit RGB values to a Vec3
///
/// # Arguments
/// - `r: u8` - red value
/// - `g: u8` - green value
/// - `b: u8` - blue value
pub fn color(r: u8, g: u8, b: u8) -> Vec3 {
    glm::vec3(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
