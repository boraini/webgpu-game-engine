use glm::Vector3;

use crate::renderer::material::{Material, PhongMaterial};

pub fn default_material() -> Material {
    Material::PhongMaterial(PhongMaterial::new_without_texture(
        Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        Vector3 {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
        Vector3 {
            x: 1.,
            y: 1.,
            z: 1.,
        },
        3.0,
    ))
}
