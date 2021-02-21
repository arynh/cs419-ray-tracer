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
use hittable::plane::Plane;
use hittable::sphere::Sphere;
use hittable::triangle::Triangle;
use hittable::Hittable;
use hittable_list::HittableList;
use light::Light;
use material::lambertian::Lambertian;
use rand::prelude::thread_rng as rng;
use rand::Rng;
use ray::Ray;

enum CameraProjection {
    Orthographic,
    Perspective,
}

// constants for image specifications
// Change these to change the image!
const IMAGE_WIDTH: u32 = 1920 / 2;
const IMAGE_HEIGHT: u32 = 1080 / 2;
const SAMPLES_LEVEL: usize = 4;
const EPSILON: f32 = 0.00001;
const MAX_HIT_DISTANCE: f32 = f32::INFINITY;
const AMBIENT_WEIGHT: f32 = 0.2;
const DIFFUSE_WEIGHT: f32 = 0.8;
const CAMERA_TYPE: CameraProjection = CameraProjection::Perspective;

fn main() {
    // configure camera position
    let mut camera_origin: Vec3 = glm::vec3(0.0, 0.0, 0.0);
    let mut camera_lookat: Vec3 = glm::vec3(0.0, 0.0, -1.0);
    let camera_up: Vec3 = glm::vec3(0.0, 1.0, 1.0);

    // create a camera
    let ortho = &OrthographicCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        90.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );
    let mut perspective = &PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        90.0,
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

    // configure object colors
    let ground_plane_color = color(58, 222, 99);
    let little_ball_color = color(194, 90, 250);
    let ground_ball_color = color(242, 78, 190);
    let triangle_color = color(242, 181, 75);

    // create world and populate it with objects
    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: glm::vec3(0.2, 0.4, -1.0),
        radius: 0.4,
        material: Box::new(Lambertian {
            albedo: little_ball_color,
        }),
    }));
    world.add(Box::new(Sphere {
        center: glm::vec3(0.0, -5.5, -3.0),
        radius: 5.0,
        material: Box::new(Lambertian {
            albedo: ground_ball_color,
        }),
    }));
    world.add(Box::new(Triangle {
        vertices: (
            glm::vec3(0.5, -0.5, -1.0),
            glm::vec3(-0.5, 1.0, -2.0),
            glm::vec3(-1.5, -0.2, -1.0),
        ),
        material: Box::new(Lambertian {
            albedo: triangle_color,
        }),
    }));
    world.add(Box::new(Plane {
        center: glm::vec3(0.0, -1.0, 0.0),
        normal: glm::vec3(0.0, 1.0, 0.0),
        material: Box::new(Lambertian {
            albedo: ground_plane_color,
        }),
    }));

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

    let mut animated_camera = PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        90.0,
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
    );

    for frame in 0..(30 * 6) {
        let theta = (2.0 * std::f32::consts::PI * frame as f32) / (30.0 * 6.0);
        let new_position =
            2.0 * glm::vec3(theta.sin(), 0.0, theta.cos()) + glm::vec3(0.0, 0.0, -1.0);

        animated_camera.move_camera(
            new_position,
            camera_lookat,
            new_position + camera_up,
            90.0,
            IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
        );

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
                    let r = animated_camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, &lights);
                }
            }
            *pixel = vec3_to_rgb(pixel_color);
        }

        img.save(format!("frames/out-{:04}.png", frame)).unwrap();
        println!("Done with frame {}!", frame);
    }
}

fn color(r: u8, g: u8, b: u8) -> Vec3 {
    glm::vec3(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

/// Shuffle the samples. Use multi-jittered
/// sampling to acheive a good distribution.
/// This method follows the presentation in:
/// https://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf
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

fn ray_color(ray: &Ray, world: &HittableList, lights: &Vec<Light>) -> Vec3 {
    match world.hit(&ray, EPSILON, MAX_HIT_DISTANCE) {
        // if we hit something, get that object's color, shade with blinn-phong
        Some(hit) => {
            let mut total = hit.material.color() * AMBIENT_WEIGHT;
            for light in lights {
                total += light.weight * light.shade_diffuse(&hit, &world);
            }
            total
        }
        // if we hit nothing, give the sky's color
        None => {
            let t = 0.5 * (glm::normalize(&ray.direction).y + 1.0);
            glm::vec3(1.0, 1.0, 1.0) * (1.0 - t) + glm::vec3(0.5, 0.7, 1.0) * t
        }
    }
}
