use crate::{renderer::material::{Material, PhongMaterial}, scene::{mesh::Mesh, object3d::Object3D}};

static WIDTH: f32 = 0.05;
static LENGTH: f32 = 5.0;

static VERTEX_ARRAY: [f32; 12 * 8] = [
    -WIDTH, 0.0, -LENGTH, 1.0,
    0.0, 0.0, 0.1, 0.0, // normal
    0.0, 0.0, 0.0, 0.0, // uv

    -WIDTH, 0.0, LENGTH, 1.0,
    0.0, 0.0, 0.1, 0.0, // normal
    0.0, 0.0, 0.0, 0.0, // uv

    WIDTH, 0.0, -LENGTH, 1.0,
    0.0, 0.0, 0.1, 0.0, // normal
    0.0, 0.0, 0.0, 0.0, // uv

    WIDTH, 0.0, LENGTH, 1.0,
    0.0, 0.0, 0.1, 0.0, // normal
    0.0, 0.0, 0.0, 0.0, // uv

    -LENGTH, 0.0, -WIDTH, 1.0,
    0.0, 0.0, 0.1, 0.0, // normal
    0.0, 0.0, 0.0, 0.0, // uv

    -LENGTH, 0.0, WIDTH, 1.0,
    0.0, 0.0, 0.1, 0.0, // normal
    0.0, 0.0, 0.0, 0.0, // uv

    LENGTH, 0.0, -WIDTH, 1.0,
    0.0, 0.0, 0.1, 0.0, // normal
    0.0, 0.0, 0.0, 0.0, // uv

    LENGTH, 0.0, WIDTH, 1.0,
    0.0, 0.0, 0.1, 0.0, // normal
    0.0, 0.0, 0.0, 0.0, // uv
];

const INDEX_ARRAY: [u32; 24] = [
    0, 1, 2, 1, 2, 3,
    0, 2, 1, 1, 3, 2,
    4, 5, 6, 5, 6, 7,
    4, 6, 5, 5, 7, 6,
];

pub fn simple_axes() -> Object3D {
    return Mesh::new_object_3d(
        Some("simple_axes".to_string()), 
        Box::new(VERTEX_ARRAY), 
        Box::new(INDEX_ARRAY),
        Material::PhongMaterial(PhongMaterial::new_without_texture(
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
            1.0
        ))
    );
}