use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::bvh::BVH;
use crate::hittable::triangle::Triangle;
use crate::hittable::Hittable;
use crate::material::lambertian::Lambertian;
use crate::ray::Ray;
use glm::Vec3;

pub struct Mesh {
    bvh: BVH,
}

impl Hittable for Mesh {
    /// If a mesh will be hit by a ray in a certain range, return a
    /// hit record with the intersection information. Otherwise, return `None`.
    /// For this triangular mesh, the hit is forwarded to the BVH containing
    /// all the triangles.
    ///
    /// # Arguments
    /// - `ray` the ray to search for intersections along
    /// - `min_distance` the minimum distance of intersections along the ray
    /// - `max_distance` the maximum distance of intersections
    ///
    /// # Returns
    /// - Optional `HitRecord` if there was a hit, otherwise `None`.
    fn hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<HitRecord> {
        self.bvh.hit(ray, min_distance, max_distance)
    }

    /// Compute the bounding box of this mesh.
    fn bounding_box(&self) -> Option<AABB> {
        self.bvh.bounding_box()
    }
}

impl Mesh {
    pub fn create(filename: String, base_color: Vec3, bvh_leaf_max: usize) -> Mesh {
        // load obj from file, triangulate faces
        let obj = tobj::load_obj(filename, true);
        assert!(obj.is_ok());
        let (models, _) = obj.unwrap();

        // get position and index buffers
        let positions = &models[0].mesh.positions;
        let indices = &models[0].mesh.indices;
        // make sure there are a whole number of triangles
        assert!(indices.len() % 3 == 0);
        assert!(positions.len() % 9 == 0);

        // create a triangle for each face
        let mut triangles: Vec<Box<dyn Hittable>> = Vec::new();
        let triangle_count = indices.len() / 3;
        println!("loading {} triangles . . . ", triangle_count);
        for tri_index in 0..triangle_count {
            let index_one = tri_index;
            let index_two = tri_index + 1;
            let index_three = tri_index + 2;
            triangles.push(Box::new(Triangle {
                vertices: [
                    glm::vec3(
                        positions[index_one],
                        positions[index_one + 1],
                        positions[index_one + 2],
                    ),
                    glm::vec3(
                        positions[index_two],
                        positions[index_two + 1],
                        positions[index_two + 2],
                    ),
                    glm::vec3(
                        positions[index_three],
                        positions[index_three + 1],
                        positions[index_three + 2],
                    ),
                ],
                material: Box::new(Lambertian { albedo: base_color }),
            }));
        }

        Mesh {
            bvh: BVH::build(triangles, bvh_leaf_max),
        }
    }
}
