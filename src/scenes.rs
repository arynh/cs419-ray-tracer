use crate::camera::perspective_camera::PerspectiveCamera;
use crate::color;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::mesh::Mesh;
use crate::hittable::plane::Plane;
use crate::hittable::sphere::Sphere;
use crate::hittable::triangle::Triangle;
use crate::light::Light;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::transparent::Transparent;
use crate::material::MaterialType;
use glm::Vec3;

/// Simple scene with a ground plane, two spheres, and a triangle.
///
/// # Returns
/// - The scene as a boxed hittable.
pub fn simple_primitives(
    image_width: u32,
    image_height: u32,
) -> (HittableList, PerspectiveCamera, Vec<Light>) {
    // configure object colors
    let ground_plane_color = color::color(58, 222, 99);
    let little_ball_color = color::color(194, 90, 250);
    let white = color::color(255, 255, 255);
    let ground_ball_color = color::color(242, 78, 190);
    let triangle_color = color::color(242, 181, 75);

    // create world and populate it
    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: glm::vec3(0.2, 0.4, -1.0),
        radius: 0.5,
        material: MaterialType::Transparent(Transparent {
            albedo: white,
            reflectance: 0.1,
            transmittance: 0.9,
            refractive_index: 1.3,
        }),
    }));
    world.add(Box::new(Sphere {
        center: glm::vec3(-0.5, 1.0, -2.0),
        radius: 0.6,
        material: MaterialType::Lambertian(Lambertian {
            albedo: triangle_color,
        }),
    }));
    world.add(Box::new(Sphere {
        center: glm::vec3(0.0, -5.5, -3.0),
        radius: 5.0,
        material: MaterialType::Lambertian(Lambertian {
            albedo: ground_ball_color,
        }),
    }));
    world.add(Box::new(Sphere {
        center: glm::vec3(3.0, -2.0, -7.0),
        radius: 2.0,
        material: MaterialType::Lambertian(Lambertian {
            albedo: little_ball_color,
        }),
    }));
    world.add(Box::new(Triangle::new(
        [
            glm::vec3(0.5, -0.5, -1.0),
            glm::vec3(-0.5, 0.75, -2.5),
            glm::vec3(-1.5, -0.2, -1.0),
        ],
        MaterialType::Metal(Metal {
            albedo: triangle_color,
        }),
    )));
    world.add(Box::new(Plane {
        center: glm::vec3(0.0, -1.0, 0.0),
        normal: glm::vec3(0.0, 1.0, 0.0),
        material: MaterialType::Lambertian(Lambertian {
            albedo: ground_plane_color,
        }),
    }));

    // configure camera position
    let camera_origin: Vec3 = glm::vec3(-1.0, 0.2, 4.0);
    let camera_lookat: Vec3 = glm::vec3(0.0, 0.1, 0.0);
    let camera_up: Vec3 = glm::vec3(0.0, 1.0, 0.0);

    // create a camera
    let camera = PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        35.0,
        image_width as f32 / image_height as f32,
    );

    (world, camera, Vec::new())
}

pub fn above_right_dragon(
    image_width: u32,
    image_height: u32,
) -> (Mesh, PerspectiveCamera, Vec<Light>) {
    // configure camera position
    let camera_origin: Vec3 = glm::vec3(3.0, 3.0, 3.0);
    let camera_lookat: Vec3 = glm::vec3(0.0, 0.0, 0.0);
    let camera_up: Vec3 = glm::vec3(0.0, 1.0, 0.0);

    // create a camera
    let camera = PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        18.0,
        image_width as f32 / image_height as f32,
    );

    let mesh = Mesh::create(
        "assets/dragon.obj",
        MaterialType::Transparent(Transparent {
            albedo: color::color(255, 255, 255),
            reflectance: 0.1,
            transmittance: 0.9,
            refractive_index: 1.3,
        }),
        32,
    );

    (mesh, camera, Vec::new())
}
