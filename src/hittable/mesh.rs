use crate::hit_record::HitRecord;
use crate::hittable::aabb::AABB;
use crate::hittable::bvh::BVH;
use crate::hittable::triangle::Triangle;
use crate::hittable::Hittable;
use crate::material::lambertian::Lambertian;
use crate::ray::Ray;
use glm::Vec3;

pub struct Mesh {
    triangles: BVH,
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
        self.triangles.hit(&ray, min_distance, max_distance)
    }

    /// Compute the bounding box of this mesh.
    fn bounding_box(&self) -> Option<AABB> {
        self.triangles.bounding_box()
    }
}

impl Mesh {
    pub fn create(filename: &str, base_color: Vec3, bvh_leaf_max: usize) -> Mesh {
        // load obj from file, triangulate faces
        let obj = tobj::load_obj(filename, true);
        assert!(obj.is_ok());
        let (models, _) = obj.unwrap();
        let model = &models[0];

        // collect all triangles
        let mut triangles: Vec<[Vec3; 3]> = Vec::new();

        // get position and index buffers
        let positions = &model.mesh.positions;
        let indices: Vec<usize> = (&model.mesh.indices).iter().map(|i| *i as usize).collect();
        // make sure there are a whole number of triangles
        assert!(indices.len() % 3 == 0);

        // create a triangle for each face
        let triangle_count = indices.len() / 3;
        println!(
            "loading {} triangles and building bvh for {} . . . ",
            triangle_count, filename
        );
        for tri_index in 0..triangle_count {
            let index_one = tri_index * 3;
            let index_two = tri_index * 3 + 1;
            let index_three = tri_index * 3 + 2;

            let vertex_one = glm::vec3(
                positions[indices[index_one] * 3],
                positions[indices[index_one] * 3 + 1],
                positions[indices[index_one] * 3 + 2],
            );
            let vertex_two = glm::vec3(
                positions[indices[index_two] * 3],
                positions[indices[index_two] * 3 + 1],
                positions[indices[index_two] * 3 + 2],
            );
            let vertex_three = glm::vec3(
                positions[indices[index_three] * 3],
                positions[indices[index_three] * 3 + 1],
                positions[indices[index_three] * 3 + 2],
            );
            triangles.push([vertex_one, vertex_two, vertex_three]);
        }

        // calculate per-vertex normals
        let mut normals: Vec<Vec3> = Vec::new();
        let vertex_count = positions.len() / 3;
        for _ in 0..vertex_count {
            normals.push(glm::vec3(0.0, 0.0, 0.0)); // initialize all to zero
        }
        for (index, triangle) in triangles.iter().enumerate() {
            let edge_one = triangle[1] - triangle[0];
            let edge_two = triangle[2] - triangle[0];
            let face_normal = glm::cross(&edge_one, &edge_two);

            let index_one = indices[index * 3];
            let index_two = indices[index * 3 + 1];
            let index_three = indices[index * 3 + 2];

            normals[index_one] += face_normal;
            normals[index_two] += face_normal;
            normals[index_three] += face_normal;
        }

        // assign the normals to triangles and make hittables
        let mut hittables: Vec<Triangle> = Vec::new();
        for (index, triangle) in triangles.iter().enumerate() {
            hittables.push(Triangle {
                vertices: *triangle,
                edges: [triangle[1] - triangle[0], triangle[2] - triangle[0]],
                vertex_normals: [
                    glm::normalize(&normals[indices[index * 3]]),
                    glm::normalize(&normals[indices[index * 3 + 1]]),
                    glm::normalize(&normals[indices[index * 3 + 2]]),
                ],
                material: Box::new(Lambertian { albedo: base_color }),
            });
        }

        Mesh {
            triangles: BVH::build(hittables, bvh_leaf_max),
        }
    }
}
