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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexBufferObject {
    pub position : glm::Vec4,
    pub normal : glm::Vec4,
    pub uv : glm::Vec4,
}

impl VertexBufferObject {
    pub fn new_from_slices(position: &[f32; 3], normal: &[f32; 3], uv: &[f32; 2]) -> Self {
        VertexBufferObject {
            position: glm::vec4(position[0], position[1], position[2], 1.0),
            normal: glm::vec4(normal[0], normal[1], normal[2], 0.0),
            uv: glm::vec4(uv[0], uv[1], 0.0, 0.0),
        }
    }
}

unsafe impl bytemuck::Zeroable for VertexBufferObject {}
unsafe impl bytemuck::Pod for VertexBufferObject {}