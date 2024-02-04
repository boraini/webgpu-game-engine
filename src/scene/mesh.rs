use super::object3d::{Object3D, Object3DObject};
use crate::{
    renderer::material::Material,
    util::{identity_matrix, MuckableMatrix},
};

#[derive(Debug)]
pub struct Mesh {
    /// draw method is triangle strip
    ///
    /// buffer layout: position(4) uv(4) texture(4) padding(4)
    pub vertex_array: Box<[f32]>,
    pub index_array: Box<[u32]>,
    pub material: Material,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
}

impl Mesh {
    pub fn new_object_3d(
        name: Option<String>,
        vertex_array: Box<[f32]>,
        index_array: Box<[u32]>,
        material: Material,
    ) -> Object3D {
        let mesh = Mesh {
            vertex_array,
            index_array,
            material,
            vertex_buffer: None,
            index_buffer: None,
        };

        Object3D {
            object: Object3DObject::Mesh(mesh),
            name,
            matrix: MuckableMatrix(identity_matrix()),
            children: vec![],
            matrix_bind_group: None,
            matrix_buffer: None,
        }
    }
}
