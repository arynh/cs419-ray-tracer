extern crate nalgebra_glm as glm;

mod camera;
mod color;
mod hit_record;
mod hittable;
mod light;
mod material;
mod ray;
mod scenes;

use atomic_counter::AtomicCounter;
use atomic_counter::RelaxedCounter;
use camera::Camera;
use glm::Vec3;
use hittable::Hittable;
use image::RgbImage;
use indicatif::ProgressBar;
use light::Light;
use material::Material;
use rand::prelude::thread_rng as rng;
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use scenes::Sky;

// constants for image specifications
// Change these to change the image!
const IMAGE_WIDTH: u32 = 1920 / 2;
const IMAGE_HEIGHT: u32 = 1080 / 2;
const SAMPLES_LEVEL: usize = 50; // samples per pixel
const DEPTH_LIMIT: u32 = 10;
const EPSILON: f32 = 0.000008;
const MAX_HIT_DISTANCE: f32 = f32::INFINITY;
const AMBIENT_WEIGHT: f32 = 0.05;
const DIFFUSE_WEIGHT: f32 = 0.8;
const SPECULAR_WEIGHT: f32 = 0.5;
const SPECULAR_COEFFICIENT: f32 = 120.0;

fn main() {
    let mut pixel_coordinates: Vec<(u32, u32)> = Vec::new();
    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            pixel_coordinates.push((x, y));
        }
    }

    println!("tracing rays . . .");
    let counter = RelaxedCounter::new(0);
    let progress_block_size: usize = 1000;
    let progress_bar =
        ProgressBar::new(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64 / progress_block_size as u64);
    let pixels: Vec<((u32, u32), Vec3)> = pixel_coordinates
        .par_iter()
        .map(|(x, y)| {
            // FIXME: some types cannot be safely shared across threads, so for
            // now, I need to recreate the scene for each task
            let (mesh, camera, lights, sky) =
                scenes::rectangle_light_example(IMAGE_WIDTH, IMAGE_HEIGHT);

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
                    pixel_color += trace_ray(&r, &mesh, &lights, &sky, DEPTH_LIMIT);
                }
            }

            // give a progress update
            counter.inc();
            let count = counter.get();
            if count % progress_block_size == 0 {
                progress_bar.inc(1);
            }

            ((*x, *y), pixel_color)
        })
        .collect();

    // convert pixel colors into 8 bit RGB pixels and place them in an image buffer
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for pixel in pixels.into_iter() {
        img.put_pixel(
            pixel.0 .0,
            pixel.0 .1,
            color::vec3_to_rgb(&pixel.1, SAMPLES_LEVEL),
        );
    }
    img.save("out.png").unwrap();
    println!("done!");
}

/// Given a ray from the camera, figure out what color that ray sees.
///
/// # Arguments
/// - `ray: &Ray` - ray along which we are sampling the scene
/// - `world: &HittableList` - objects that compose our scene
/// - `lights: &Vec<Light>` - light sources for the scene
///
///
/// # Returns
/// - `Vec3` - the color that this ray contributes to the pixel
fn trace_ray<T: Hittable>(ray: &Ray, world: &T, lights: &[Light], sky: &Sky, depth: u32) -> Vec3 {
    if depth > 0 {
        if let Some(hit) = world.hit(&ray, EPSILON, MAX_HIT_DISTANCE) {
            if let Some(material) = &hit.material {
                material.shade(world, lights, sky, &hit.ray, &hit, depth)
            } else {
                color::color(0, 0, 0)
            }
        } else {
            // if we hit nothing, give the sky's color
            sky(ray)
        }
    } else {
        color::color(0, 0, 0)
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
