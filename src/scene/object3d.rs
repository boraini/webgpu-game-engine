use std::num::NonZeroU64;

use glm::GenSquareMat;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BufferBinding, BufferDescriptor, BufferUsages,
};

use super::mesh::Mesh;
use crate::{
    renderer::draw_state::DrawState,
    renderer::material::MaterialManager,
    renderer::{
        staging_belt_and_command_encoder::StagingBeltAndCommandEncoder, wgpu_handles::WgpuHandles,
    },
    util::{identity_matrix, MuckableMatrix},
};

#[derive(Debug)]
pub enum Object3DObject {
    Empty,
    Scene,
    Mesh(Mesh),
}

#[derive(Debug)]
pub struct Object3D {
    pub name: Option<String>,
    pub matrix: MuckableMatrix,
    pub object: Object3DObject,
    pub children: Vec<Object3D>,
    pub matrix_bind_group: Option<wgpu::BindGroup>,
    pub matrix_buffer: Option<wgpu::Buffer>,
}

impl Object3D {
    pub fn create_empty() -> Self {
        Self {
            name: None,
            matrix: MuckableMatrix(identity_matrix()),
            object: Object3DObject::Empty,
            children: vec![],
            matrix_bind_group: None,
            matrix_buffer: None,
        }
    }

    pub fn is_drawable(&self) -> bool {
        matches!(&self.object, Object3DObject::Mesh(_))
    }

    /// Compute the world matrices of all objects and write them to their buffers.
    pub fn write_matrices(
        &mut self,
        wgpu_handles: &WgpuHandles,
        draw_state: &mut DrawState,
        staging_belt: &mut StagingBeltAndCommandEncoder,
    ) {
        draw_state.push_matrix();
        draw_state.transform(&self.matrix.into());

        match self.matrix_buffer.as_ref() {
            Some(buf) => {
                wgpu_handles
                    .queue
                    .write_buffer(buf, 0, bytemuck::cast_slice(&[self.matrix]))
            }
            None => {}
        }

        let matrix_buffer = self.matrix_buffer.get_or_insert_with(|| {
            wgpu_handles.device.create_buffer(&BufferDescriptor {
                label: Some("SomeMatrixBuffer"),
                size: 128,
                usage: BufferUsages::UNIFORM.union(BufferUsages::COPY_DST),
                mapped_at_creation: false,
            })
        });

        self.matrix_bind_group.get_or_insert_with(|| {
            wgpu_handles.device.create_bind_group(&BindGroupDescriptor {
                label: Some("SomeMatrixBindGroup"),
                layout: draw_state
                    .material_manager
                    .matrix_bind_group_layout
                    .as_ref()
                    .unwrap(),
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(BufferBinding {
                        buffer: matrix_buffer,
                        offset: 0,
                        size: NonZeroU64::new(128),
                    }),
                }],
            })
        });

        let matrix_to_write = draw_state.get_matrix().to_owned();
        let matrix_inverse = matrix_to_write.inverse().unwrap();
        staging_belt.write_buffer(
            self.matrix_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&[
                MuckableMatrix(matrix_to_write),
                MuckableMatrix(matrix_inverse),
            ]),
            &wgpu_handles.device,
        );

        for child in &mut self.children {
            child.write_matrices(wgpu_handles, draw_state, staging_belt);
        }

        draw_state.pop_matrix();
    }

    pub fn write_materials(
        &mut self,
        wgpu_handles: &WgpuHandles,
        material_manager: &MaterialManager,
        staging_belt: &mut StagingBeltAndCommandEncoder,
    ) {
        match &mut self.object {
            Object3DObject::Mesh(mesh) => {
                material_manager.write_material(&mut mesh.material, wgpu_handles, staging_belt);
            }
            _ => {}
        }

        for child in &mut self.children {
            child.write_materials(wgpu_handles, material_manager, staging_belt);
        }
    }
}

pub trait Object3DManager {
    fn write_object_3d_single(&self, object_3d: &mut Object3D, wgpu_handles: &WgpuHandles);

    fn write_object_3d(&self, object_3d: &mut Object3D, wgpu_handles: &WgpuHandles);
}

impl Object3DManager for MaterialManager {
    fn write_object_3d_single(&self, object_3d: &mut Object3D, wgpu_handles: &WgpuHandles) {
        match &mut object_3d.object {
            Object3DObject::Mesh(mesh) => {
                if mesh.vertex_buffer.is_some() {
                    return;
                } // we allow only writing once
                mesh.vertex_buffer.get_or_insert_with(|| {
                    wgpu_handles
                        .device
                        .create_buffer_init(&BufferInitDescriptor {
                            label: Some("SomeMeshVertexBuffer"),
                            contents: bytemuck::cast_slice(&mesh.vertex_array),
                            usage: BufferUsages::VERTEX.union(BufferUsages::COPY_DST),
                        })
                });
                mesh.index_buffer.get_or_insert_with(|| {
                    wgpu_handles
                        .device
                        .create_buffer_init(&BufferInitDescriptor {
                            label: Some("SomeMeshIndexBuffer"),
                            contents: bytemuck::cast_slice(&mesh.index_array),
                            usage: BufferUsages::INDEX.union(BufferUsages::COPY_DST),
                        })
                });
            }
            _ => {}
        }
    }

    fn write_object_3d(&self, object_3d: &mut Object3D, wgpu_handles: &WgpuHandles) {
        self.write_object_3d_single(object_3d, wgpu_handles);

        for child in &mut object_3d.children {
            self.write_object_3d(child, wgpu_handles);
        }
    }
}
