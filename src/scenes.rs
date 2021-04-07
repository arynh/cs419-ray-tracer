use crate::camera::perspective_camera::PerspectiveCamera;
use crate::color;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::mesh::Mesh;
use crate::hittable::plane::Plane;
use crate::hittable::rectangle::Rectangle;
use crate::hittable::sphere::Sphere;
use crate::hittable::triangle::Triangle;
use crate::hittable::HittableItem;
use crate::light::Light;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::transparent::Transparent;
use crate::material::MaterialType;
use crate::ray::Ray;
use glm::Vec3;
use image::GenericImageView;
use image::Pixel;

/// A sky takes a &Ray and return the color of the skybox in that ray's
/// direction.
pub type Sky = fn(&Ray) -> Vec3;

pub fn infinite_mirror_hallway(
    image_width: u32,
    image_height: u32,
) -> (HittableList, PerspectiveCamera, Vec<Light>, Sky) {
    let ground_plane_color = color::color(58, 222, 99);
    let little_ball_color = color::color(0, 255, 0);

    // create world
    let mut world = HittableList::new();
    // add rectangular mirror on either side
    world.add(HittableItem::Rectangle(Rectangle::new(
        [
            glm::vec3(-2.0, 2.0, 0.0),
            glm::vec3(-2.0, 2.0, -100.0),
            glm::vec3(-2.0, 0.0, -100.0),
            glm::vec3(-2.0, 0.0, 0.0),
        ],
        MaterialType::Metal(Metal {
            albedo: color::color(255, 255, 255),
        }),
    )));
    world.add(HittableItem::Rectangle(Rectangle::new(
        [
            glm::vec3(2.0, 2.0, 0.0),
            glm::vec3(2.0, 2.0, -100.0),
            glm::vec3(2.0, 0.0, -100.0),
            glm::vec3(2.0, 0.0, 0.0),
        ],
        MaterialType::Metal(Metal {
            albedo: color::color(255, 255, 255),
        }),
    )));
    // little ball
    world.add(HittableItem::Sphere(Sphere {
        center: glm::vec3(0.0, 1.0, -20.0),
        radius: 0.5,
        material: MaterialType::Lambertian(Lambertian {
            albedo: little_ball_color,
        }),
    }));
    // big ball
    world.add(HittableItem::Sphere(Sphere {
        center: glm::vec3(0.0, 10.0, -15.0),
        radius: 5.0,
        material: MaterialType::Metal(Metal {
            albedo: color::color(255, 255, 255),
        }),
    }));

    // configure camera position
    let camera_origin: Vec3 = glm::vec3(0.0, 1.0, 1.0);
    let camera_lookat: Vec3 = glm::vec3(0.0, 1.1, 0.0);
    let camera_up: Vec3 = glm::vec3(0.0, 1.0, 0.0);

    // create a camera
    let camera = PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        35.0,
        image_width as f32 / image_height as f32,
    );

    let sunset_sky_gradient = |ray: &Ray| {
        let t = ray.direction.x;
        0.5 * color::color(245, 64, 64) * (1.0 - t) + 1.5 * color::color(255, 201, 34) * t
    };

    (world, camera, Vec::new(), sunset_sky_gradient)
}

/// Simple scene with a ground plane, two spheres, and a triangle.
///
/// # Returns
/// - The scene as a boxed hittable.
pub fn simple_primitives(
    image_width: u32,
    image_height: u32,
) -> (HittableList, PerspectiveCamera, Vec<Light>, Sky) {
    // configure object colors
    let ground_plane_color = color::color(58, 222, 99);
    let little_ball_color = color::color(194, 90, 250);
    let white = color::color(255, 255, 255);
    let ground_ball_color = color::color(242, 78, 190);
    let triangle_color = color::color(242, 181, 75);

    // create world and populate it
    let mut world = HittableList::new();
    world.add(HittableItem::Sphere(Sphere {
        center: glm::vec3(0.2, 0.4, -1.0),
        radius: 0.5,
        material: MaterialType::Transparent(Transparent {
            albedo: white,
            reflectance: 0.1,
            transmittance: 0.9,
            refractive_index: 1.3,
        }),
    }));
    world.add(HittableItem::Sphere(Sphere {
        center: glm::vec3(-0.5, 1.0, -2.0),
        radius: 0.6,
        material: MaterialType::Lambertian(Lambertian {
            albedo: triangle_color,
        }),
    }));
    world.add(HittableItem::Sphere(Sphere {
        center: glm::vec3(0.0, -5.5, -3.0),
        radius: 5.0,
        material: MaterialType::Lambertian(Lambertian {
            albedo: ground_ball_color,
        }),
    }));
    world.add(HittableItem::Sphere(Sphere {
        center: glm::vec3(3.0, -2.0, -7.0),
        radius: 2.0,
        material: MaterialType::Lambertian(Lambertian {
            albedo: little_ball_color,
        }),
    }));
    world.add(HittableItem::Triangle(Triangle::new(
        [
            glm::vec3(0.5, -0.5, -1.0),
            glm::vec3(-0.5, 0.75, -2.5),
            glm::vec3(-1.5, -0.2, -1.0),
        ],
        MaterialType::Metal(Metal {
            albedo: triangle_color,
        }),
    )));
    world.add(HittableItem::Plane(Plane {
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

    let sunset_sky_gradient = |ray: &Ray| {
        let t = ray.direction.x;
        0.5 * color::color(245, 64, 64) * (1.0 - t) + 1.5 * color::color(255, 201, 34) * t
    };

    // TODO: currently, this architecture doesn't support skyboxes
    // // load the skybox
    // let sky = |ray: &Ray| {
    //     let sky_image = image::open("assets/outside.jpg").unwrap();
    //     let (sky_width, sky_height) = sky_image.dimensions();
    //     let horizontal_angle = (-ray.direction.z / ray.direction.x).atan();
    //     let vertical_angle = (ray.direction.y / -ray.direction.z).atan();
    //     let u = horizontal_angle / std::f32::consts::PI;
    //     let v = vertical_angle / std::f32::consts::PI;
    //     let x = (u * (sky_width as f32 / 2.0) + (sky_width as f32 / 2.0)) as u32;
    //     let y = (v * (sky_height as f32 / 2.0) + (sky_height as f32 / 2.0)) as u32;
    //     let image_pixel = sky_image.get_pixel(x, y);
    //     let pixel = image_pixel.channels();
    //     glm::vec3(
    //         pixel[0] as f32 / 255.0,
    //         pixel[1] as f32 / 255.0,
    //         pixel[2] as f32 / 255.0,
    //     )
    // };

    (world, camera, Vec::new(), sunset_sky_gradient)
}

pub fn rectangle_light_example(
    image_width: u32,
    image_height: u32,
) -> (HittableList, PerspectiveCamera, Vec<Light>, Sky) {
    // configure object colors
    let ground_plane_color = color::color(58, 222, 99);
    let little_ball_color = color::color(194, 90, 250);
    let white = color::color(255, 255, 255);

    // create world and populate it
    let mut world = HittableList::new();
    world.add(HittableItem::Sphere(Sphere {
        center: glm::vec3(2.0, 0.5, -3.5),
        radius: 0.5,
        material: MaterialType::Lambertian(Lambertian {
            albedo: little_ball_color,
        }),
    }));
    world.add(HittableItem::Sphere(Sphere {
        center: glm::vec3(4.0, 0.0, -3.0),
        radius: 0.5,
        material: MaterialType::Transparent(Transparent {
            albedo: white,
            reflectance: 0.1,
            transmittance: 0.9,
            refractive_index: 1.3,
        }),
    }));
    world.add(HittableItem::Plane(Plane {
        center: glm::vec3(0.0, -1.0, 0.0),
        normal: glm::vec3(0.0, 1.0, 0.0),
        material: MaterialType::Lambertian(Lambertian {
            albedo: ground_plane_color,
        }),
    }));
    // add an area light
    world.add(HittableItem::Rectangle(Rectangle::new(
        [
            glm::vec3(3.0, 2.0, -2.0),
            glm::vec3(5.0, 2.0, -2.0),
            glm::vec3(5.0, 2.0, -4.0),
            glm::vec3(3.0, 2.0, -4.0),
        ],
        MaterialType::DiffuseLight(DiffuseLight { color: 5.0 * white }),
    )));

    // configure camera position
    let camera_origin: Vec3 = glm::vec3(-1.0, 0.2, 2.0);
    let camera_lookat: Vec3 = glm::vec3(0.1, 0.3, 0.0);
    let camera_up: Vec3 = glm::vec3(0.0, 1.0, 0.0);

    // create a camera
    let camera = PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        35.0,
        image_width as f32 / image_height as f32,
    );

    let sunset_sky_gradient = |ray: &Ray| {
        let t = ray.direction.x;
        0.1 * (0.5 * color::color(245, 64, 64) * (1.0 - t) + 1.5 * color::color(255, 201, 34) * t)
    };

    (world, camera, Vec::new(), sunset_sky_gradient)
}

pub fn teapot_caustic(
    image_width: u32,
    image_height: u32,
) -> (HittableList, PerspectiveCamera, Vec<Light>, Sky) {
    // configure camera position
    let camera_origin: Vec3 = glm::vec3(5.0, 2.0, 20.0);
    let camera_lookat: Vec3 = glm::vec3(0.0, 1.5, 0.0);
    let camera_up: Vec3 = glm::vec3(0.0, 1.0, 0.0);

    // create a camera
    let camera = PerspectiveCamera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        30.0,
        image_width as f32 / image_height as f32,
    );

    let mesh = Mesh::create(
        "assets/teapot.obj",
        MaterialType::Transparent(Transparent {
            albedo: color::color(255, 255, 255),
            reflectance: 0.1,
            transmittance: 0.9,
            refractive_index: 1.3,
        }),
        // MaterialType::Lambertian(Lambertian {
        //     albedo: color::color(128, 128, 128),
        // }),
        32,
    );

    let mut world = HittableList::new();
    // teapot
    world.add(HittableItem::Mesh(mesh));
    // ground plane
    world.add(HittableItem::Plane(Plane {
        center: glm::vec3(0.0, -1.0, 0.0),
        normal: glm::vec3(0.0, 1.0, 0.0),
        material: MaterialType::Lambertian(Lambertian {
            albedo: color::color(128, 128, 128),
        }),
    }));
    // area light
    world.add(HittableItem::Rectangle(Rectangle::new(
        [
            glm::vec3(-3.0, 5.0, -3.0),
            glm::vec3(3.0, 5.0, -3.0),
            glm::vec3(3.0, 5.0, 3.0),
            glm::vec3(-3.0, 5.0, 3.0),
        ],
        MaterialType::DiffuseLight(DiffuseLight {
            color: 5.0 * color::color(255, 255, 255),
        }),
    )));

    let sunset_sky_gradient = |ray: &Ray| {
        let t = ray.direction.x;
        0.1 * (0.5 * color::color(245, 64, 64) * (1.0 - t) + 1.5 * color::color(255, 201, 34) * t)
    };

    (world, camera, Vec::new(), sunset_sky_gradient)
}

pub fn above_right_dragon(
    image_width: u32,
    image_height: u32,
) -> (Mesh, PerspectiveCamera, Vec<Light>, Sky) {
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

    let gentle_red_gradient_sky = |ray: &Ray| {
        let t = ray.direction.x;
        color::color(245, 64, 64) * (1.0 - t * t) + 1.5 * color::color(255, 255, 255) * t * t
    };

    (mesh, camera, Vec::new(), gentle_red_gradient_sky)
}
