extern crate image;
extern crate nalgebra_glm as glm;

mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod material;
mod ray;

use camera::perspective_camera::PerspectiveCamera;
use camera::Camera;
use glm::Vec3;
use hittable::sphere::Sphere;
use hittable::Hittable;
use hittable_list::HittableList;
use material::lambertian::Lambertian;
use rand::Rng;
use ray::Ray;

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        glm::vec3(0.0, 0.0, 0.0)
    } else {
        match world.hit(&ray, 0.0001, std::f32::INFINITY) {
            // if we hit something, get that object's color
            Some(hit) => match hit.material.scatter(&ray, &hit) {
                Some(new_ray) => {
                    let new_color = glm::matrix_comp_mult(
                        &(new_ray.attenuation),
                        &ray_color(&new_ray, world, depth - 1),
                    );
                    return new_color;
                }
                None => glm::vec3(0.0, 0.0, 0.0),
            },
            // if we hit nothing, give the sky's color
            None => {
                let t = 0.5 * (glm::normalize(&ray.direction).y + 1.0);
                glm::vec3(1.0, 1.0, 1.0) * (1.0 - t) + glm::vec3(0.5, 0.7, 1.0) * t
            }
        }
    }
}

fn main() {
    let image_width = 1920 / 2;
    let image_height = 1080 / 2;
    let samples_per_pixel = 20;
    let max_depth = 20;

    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: glm::vec3(0.2, 0.5, -1.0),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: glm::vec3(0.7, 0.3, 0.3),
        }),
    }));
    world.add(Box::new(Sphere {
        center: glm::vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: glm::vec3(0.8, 0.8, 0.0),
        }),
    }));

    let camera = PerspectiveCamera::new_default_perspective();

    let vec3_to_rgb = |vec: Vec3| {
        let scaled = vec / samples_per_pixel as f32;
        let clamped = glm::clamp(&scaled, 0.0, 1.0);
        let converted = clamped * 255.0;
        image::Rgb([converted.x as u8, converted.y as u8, converted.z as u8])
    };

    let img = image::ImageBuffer::from_fn(image_width, image_height, |i, j| {
        let mut pixel_color = glm::vec3(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u = (i as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
            let v =
                ((image_height - j - 1) as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
            let r = camera.get_ray(u, v);
            pixel_color += ray_color(&r, &world, max_depth);
        }
        vec3_to_rgb(pixel_color)
    });

    img.save("out.png").unwrap();
}
