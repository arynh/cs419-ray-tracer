extern crate image;
extern crate nalgebra_glm as glm;

mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod light;
mod material;
mod ray;

use camera::orthographic_camera::OrthographicCamera;
use camera::perspective_camera::PerspectiveCamera;
use camera::Camera;
use glm::Vec3;
use hittable::bvh::BVH;
use hittable::mesh::Mesh;
use hittable::sphere::Sphere;
use hittable::triangle::Triangle;
use hittable::Hittable;
use hittable_list::HittableList;
use light::Light;
use material::lambertian::Lambertian;
use rand::prelude::thread_rng as rng;
use rand::Rng;
use ray::Ray;

// Camera projection types used for configuring which camera to use
enum CameraProjection {
    Orthographic,
    Perspective,
}

// constants for image specifications
// Change these to change the image!
const IMAGE_WIDTH: u32 = 960;
const IMAGE_HEIGHT: u32 = 540;
const SAMPLES_LEVEL: usize = 1; // SAMPLES_LEVEL^2 samples per pixel
const EPSILON: f32 = 0.0001;
const MAX_HIT_DISTANCE: f32 = f32::INFINITY;
const AMBIENT_WEIGHT: f32 = 0.05;
const DIFFUSE_WEIGHT: f32 = 0.8;
const SPECULAR_WEIGHT: f32 = 0.4;
const SPECULAR_COEFFICIENT: f32 = 120.0;
const CAMERA_TYPE: CameraProjection = CameraProjection::Perspective;

fn main() {
    // configure camera position
    let camera_origin: Vec3 = glm::vec3(0.0, 0.0, 1.0);
    let camera_lookat: Vec3 = glm::vec3(0.0, 0.0, -1.0);
    let camera_up: Vec3 = glm::vec3(0.0, 1.0, 0.0);

    // create a camera
    let ortho = &OrthographicCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        45.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );
    let perspective = &PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        45.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );
    let camera: &dyn Camera = match CAMERA_TYPE {
        CameraProjection::Orthographic => {
            drop(perspective);
            ortho
        }
        CameraProjection::Perspective => {
            drop(ortho);
            perspective
        }
    };

    // create world and populate it with objects
    let x_extent = 90.0;
    let y_extent = 50.0;
    let z_close = -125.0;
    let z_far = z_close - 30.0;
    let num_spheres = 10_000;
    let mut world = HittableList::new();
    for _ in 0..num_spheres {
        world.add(Box::new(Sphere {
            center: glm::vec3(
                2.0 * x_extent * rng().gen::<f32>() - x_extent,
                2.0 * y_extent * rng().gen::<f32>() - y_extent,
                (z_far - z_close) * rng().gen::<f32>() + z_close,
            ),
            radius: 1.0,
            material: Box::new(Lambertian {
                albedo: glm::vec3(rng().gen::<f32>(), rng().gen::<f32>(), rng().gen::<f32>()),
            }),
        }));
    }

    println!("building bvh . . .");
    let bvh = BVH::build(world.objects, 20);

    // create light source vector
    let point_light1 = Light {
        position: glm::vec3(50.0, 50.0, -50.0),
        weight: 1.0,
    };
    let lights = vec![point_light1];

    // convert from vector to adjusted and clamped RBG values
    let vec3_to_rgb = |vec: Vec3| {
        let scaled = vec / (SAMPLES_LEVEL * SAMPLES_LEVEL) as f32;
        let clamped = glm::clamp(&scaled, 0.0, 1.0);
        let converted = clamped * 255.0;
        image::Rgb([converted.x as u8, converted.y as u8, converted.z as u8])
    };

    // preallocate an array for the multi-jittered sampling
    let mut jitter_boxes: [[(f32, f32); SAMPLES_LEVEL]; SAMPLES_LEVEL] =
        [[(0.0, 0.0); SAMPLES_LEVEL]; SAMPLES_LEVEL];
    // initialize the canonical arrangement for multi-jittered sampling
    for j in 0..SAMPLES_LEVEL {
        for i in 0..SAMPLES_LEVEL {
            let j_float = j as f32;
            let i_float = i as f32;
            let n_float = SAMPLES_LEVEL as f32;
            jitter_boxes[j][i].0 = (i_float + (j_float + rng().gen::<f32>()) / n_float) / n_float;
            jitter_boxes[j][i].1 = (j_float + (i_float + rng().gen::<f32>()) / n_float) / n_float;
        }
    }
    println!("tracing rays . . .");

    let mut img = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let image_width = IMAGE_WIDTH as f32 - 1.0;
    let image_height = IMAGE_HEIGHT as f32 - 1.0;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color = glm::vec3(0.0, 0.0, 0.0);
        let jitter_boxes = shuffle_jittered_sampling(&mut jitter_boxes);
        let x_float = x as f32;
        let y_float = image_height - y as f32;
        for j in 0..SAMPLES_LEVEL {
            for i in 0..SAMPLES_LEVEL {
                let u = (x_float + jitter_boxes[j][i].0) / image_width;
                let v = (y_float + jitter_boxes[j][i].1) / image_height;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &bvh, &lights);
            }
        }
        *pixel = vec3_to_rgb(pixel_color);
    }
    img.save("out.png").unwrap();
}

/// Given a ray from the camera, figure out what color that ray sees.
///
/// # Arguments
/// - `ray: &Ray` - ray along which we are sampling the scene
/// - `world: &HittableList` - objects that compose our scene
/// - `lights: &Vec<Light>` - light sources for the scene
///
/// # Returns
/// - `Vec3` - the color that this ray contributes to the pixel
fn ray_color(ray: &Ray, world: &dyn Hittable, lights: &Vec<Light>) -> Vec3 {
    match world.hit(&ray, EPSILON, MAX_HIT_DISTANCE) {
        // if we hit something, get that object's color, shade with blinn-phong
        Some(hit) => {
            let color = hit.material.unwrap().color();
            let mut total = color * AMBIENT_WEIGHT;
            for light in lights {
                total += light.weight * light.shade(&hit, world);
            }
            let g = 1.0 / 2.2;
            glm::pow(&total, &glm::vec3(g, g, g))
        }
        // if we hit nothing, give the sky's color
        None => {
            let t = 0.5 * (glm::normalize(&ray.direction).y + 1.0);
            glm::vec3(1.0, 1.0, 1.0) * (1.0 - t) + glm::vec3(0.5, 0.7, 1.0) * t
        }
    }
}

/// Shuffle the samples. Use multi-jittered sampling to acheive a good
/// distribution. This method follows the presentation in:
/// https://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf
///
/// # Arguments
/// - `jitter_boxes: &mut [[(f32, f32); SAMPLES_LEVEL]; SAMPLES_LEVEL]` - the
///     array of sample locations arranged on the NxN grid. This is a mutable
///     reference, so values are edited in place.
///
/// # Returns
/// - `&[[(f32, f32); SAMPLES_LEVEL]; SAMPLES_LEVEL]` - the reference to the
///     sample locations to return ownership to the main loop
fn shuffle_jittered_sampling(
    jitter_boxes: &mut [[(f32, f32); SAMPLES_LEVEL]; SAMPLES_LEVEL],
) -> &[[(f32, f32); SAMPLES_LEVEL]; SAMPLES_LEVEL] {
    for j in 0..SAMPLES_LEVEL {
        for i in 0..SAMPLES_LEVEL {
            let k: usize = ((j as f32 + rng().gen::<f32>() * (SAMPLES_LEVEL - j) as f32) as usize)
                .min(SAMPLES_LEVEL - 1);
            let temp = jitter_boxes[j][i].0;
            jitter_boxes[j][i].0 = jitter_boxes[k][i].0;
            jitter_boxes[k][i].0 = temp;
        }
    }
    for i in 0..SAMPLES_LEVEL {
        for j in 0..SAMPLES_LEVEL {
            let k: usize = ((i as f32 + rng().gen::<f32>() * (SAMPLES_LEVEL - i) as f32) as usize)
                .min(SAMPLES_LEVEL - 1);
            let temp = jitter_boxes[j][i].1;
            jitter_boxes[j][i].1 = jitter_boxes[j][k].1;
            jitter_boxes[j][k].0 = temp;
        }
    }
    jitter_boxes
}

/// Utility to convert from 8 bit RGB values to a Vec3
///
/// # Arguments
/// - `r: u8` - red value
/// - `g: u8` - green value
/// - `b: u8` - blue value
fn color(r: u8, g: u8, b: u8) -> Vec3 {
    glm::vec3(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
