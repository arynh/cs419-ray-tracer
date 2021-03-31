extern crate image;
extern crate nalgebra_glm as glm;

mod camera;
mod hit_record;
mod hittable;
mod light;
mod material;
mod ray;

use camera::perspective_camera::PerspectiveCamera;
use camera::Camera;
use glm::Vec3;
use hittable::hittable_list::HittableList;
use hittable::mesh::Mesh;
use hittable::plane::Plane;
use hittable::sphere::Sphere;
use hittable::triangle::Triangle;
use hittable::Hittable;
use light::Light;
use material::lambertian::Lambertian;
use material::Material;
use material::MaterialType;
use rand::prelude::thread_rng as rng;
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;

// Camera projection types used for configuring which camera to use
enum CameraProjection {
    Orthographic,
    Perspective,
}

// constants for image specifications
// Change these to change the image!
const IMAGE_WIDTH: u32 = 500;
const IMAGE_HEIGHT: u32 = 500;
const SAMPLES_LEVEL: usize = 20; // SAMPLES_LEVEL^2 samples per pixel
const DEPTH_LIMIT: usize = 100;
const EPSILON: f32 = 0.000008;
const MAX_HIT_DISTANCE: f32 = f32::INFINITY;
const AMBIENT_WEIGHT: f32 = 0.05;
const DIFFUSE_WEIGHT: f32 = 0.8;
const SPECULAR_WEIGHT: f32 = 0.5;
const SPECULAR_COEFFICIENT: f32 = 120.0;

fn main() {
    // configure camera position
    let camera_origin: Vec3 = glm::vec3(-1.0, 0.2, 4.0);
    let camera_lookat: Vec3 = glm::vec3(0.0, 0.1, 0.0);
    let camera_up: Vec3 = glm::vec3(0.0, 1.0, 0.0);

    // create a camera
    let camera = PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        25.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );

    let mesh = Mesh::create("assets/dragon.obj", color(255, 255, 255), 32);

    // configure object colors
    let ground_plane_color = color(58, 222, 99);
    let little_ball_color = color(194, 90, 250);
    let ground_ball_color = color(242, 78, 190);
    let triangle_color = color(242, 181, 75);

    // create light source vector
    let point_light1 = Light {
        position: glm::vec3(1.0, 2.0, 1.0),
        weight: 0.5,
    };
    let point_light2 = Light {
        position: glm::vec3(-1.0, 2.0, 1.0),
        weight: 0.4,
    };
    let lights = vec![point_light1, point_light2];

    let mut pixel_coordinates: Vec<(u32, u32)> = Vec::new();
    for x in (0..IMAGE_WIDTH).rev() {
        for y in (0..IMAGE_HEIGHT).rev() {
            pixel_coordinates.push((x, y));
        }
    }

    println!("tracing rays . . .");
    let pixels: Vec<((u32, u32), Vec3)> = pixel_coordinates
        .par_iter()
        .map(|(x, y)| {
            // // create world and populate it with objects
            // let mut world = HittableList::new();
            // world.add(Box::new(Sphere {
            //     center: glm::vec3(0.2, 0.4, -1.0),
            //     radius: 0.4,
            //     material: MaterialType::Lambertian(Lambertian {
            //         albedo: little_ball_color,
            //     }),
            // }));
            // world.add(Box::new(Sphere {
            //     center: glm::vec3(0.0, -5.5, -3.0),
            //     radius: 5.0,
            //     material: MaterialType::Lambertian(Lambertian {
            //         albedo: ground_ball_color,
            //     }),
            // }));
            // world.add(Box::new(Triangle::new(
            //     [
            //         glm::vec3(0.5, -0.5, -1.0),
            //         glm::vec3(-0.5, 1.0, -2.0),
            //         glm::vec3(-1.5, -0.2, -1.0),
            //     ],
            //     MaterialType::Lambertian(Lambertian {
            //         albedo: triangle_color,
            //     }),
            // )));
            // world.add(Box::new(Plane {
            //     center: glm::vec3(0.0, -1.0, 0.0),
            //     normal: glm::vec3(0.0, 1.0, 0.0),
            //     material: MaterialType::Lambertian(Lambertian {
            //         albedo: ground_plane_color,
            //     }),
            // }));

            // preallocate an array for the multi-jittered sampling
            let mut jitter_boxes: [[(f32, f32); SAMPLES_LEVEL]; SAMPLES_LEVEL] =
                [[(0.0, 0.0); SAMPLES_LEVEL]; SAMPLES_LEVEL];
            // initialize the canonical arrangement for multi-jittered sampling
            for j in 0..SAMPLES_LEVEL {
                for i in 0..SAMPLES_LEVEL {
                    let j_float = j as f32;
                    let i_float = i as f32;
                    let n_float = SAMPLES_LEVEL as f32;
                    jitter_boxes[j][i].0 =
                        (i_float + (j_float + rng().gen::<f32>()) / n_float) / n_float;
                    jitter_boxes[j][i].1 =
                        (j_float + (i_float + rng().gen::<f32>()) / n_float) / n_float;
                }
            }

            let image_width = IMAGE_WIDTH as f32 - 1.0;
            let image_height = IMAGE_HEIGHT as f32 - 1.0;
            let mut pixel_color = glm::vec3(0.0, 0.0, 0.0);
            let jitter_boxes = shuffle_jittered_sampling(&mut jitter_boxes);
            let x_float = *x as f32;
            let y_float = image_height - *y as f32;
            for j in 0..SAMPLES_LEVEL {
                for i in 0..SAMPLES_LEVEL {
                    let u = (x_float + jitter_boxes[j][i].0) / image_width;
                    let v = (y_float + jitter_boxes[j][i].1) / image_height;
                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &mesh, &lights, DEPTH_LIMIT);
                }
            }
            ((*x, *y), pixel_color)
        })
        .collect();

    // convert pixel colors into 8 bit RGB pixels and place them in an image buffer
    let mut img = image::RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for pixel in pixels.into_iter() {
        img.put_pixel(pixel.0 .0, pixel.0 .1, vec3_to_rgb(&pixel.1));
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
fn ray_color<T: Hittable>(ray: &Ray, world: &T, lights: &[Light], depth: usize) -> Vec3 {
    if depth > 0 {
        if let Some(hit) = world.hit(&ray, EPSILON, MAX_HIT_DISTANCE) {
            if let Some(scattered_ray) = &hit.material.unwrap().scatter(&ray, &hit) {
                glm::matrix_comp_mult(
                    &(scattered_ray.attenuation.unwrap()),
                    &ray_color(&scattered_ray, world, &lights, depth - 1),
                )
            } else {
                glm::vec3(0.0, 0.0, 0.0)
            }
        } else {
            // if we hit nothing, give the sky's color
            let t = glm::normalize(&ray.direction).x;
            0.5 * color(245, 64, 64) * (1.0 - t) + 1.5 * color(255, 201, 34) * t
        }
    } else {
        glm::vec3(0.0, 0.0, 0.0)
    }
}

/// Shuffle the samples. Use multi-jittered sampling to acheive a good
/// distribution. This method follows the presentation in:
/// https://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf
///
/// # Arguments
/// - `jitter_boxes: &mut [[[f32; 2]; SAMPLES_LEVEL]; SAMPLES_LEVEL]` - the
///     array of sample locations arranged on the NxN grid. This is a mutable
///     reference, so values are edited in place.
///
/// # Returns
/// - `&[[[f32; 2]; SAMPLES_LEVEL]; SAMPLES_LEVEL]` - the reference to the
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

/// Convert from vector to gamma adjusted and clamped RGB values.
///
/// # Arguments
/// - `vec: &Vec3` - Vec3 to convert to a RGB pixel
fn vec3_to_rgb(vec: &Vec3) -> image::Rgb<u8> {
    let scaled = vec / (SAMPLES_LEVEL * SAMPLES_LEVEL) as f32;
    let g = 1.0 / 2.2;
    let adjusted = glm::pow(&scaled, &glm::vec3(g, g, g));
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
fn color(r: u8, g: u8, b: u8) -> Vec3 {
    glm::vec3(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
