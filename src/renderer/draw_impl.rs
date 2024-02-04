use super::bind_material::BindMaterial;
use crate::renderer::draw_state::DrawState;
use crate::scene::{mesh::Mesh, object3d::Object3D};
use wgpu::{self, RenderPass};

pub trait DrawObject3D<'a> {
    fn draw_object_3d(&mut self, draw_state: &mut DrawState, mesh: &'a Object3D);
}

impl<'a> DrawObject3D<'a> for RenderPass<'a> {
    fn draw_object_3d(&mut self, draw_state: &mut DrawState, object3d: &'a Object3D) {
        if object3d.is_drawable() {
            self.set_bind_group(0, object3d.matrix_bind_group.as_ref().unwrap(), &[]);
        }

        match &object3d.object {
            crate::scene::object3d::Object3DObject::Empty => {}
            crate::scene::object3d::Object3DObject::Scene => {}
            crate::scene::object3d::Object3DObject::Mesh(mesh) => self.draw_mesh(draw_state, mesh),
        };

        for child in &object3d.children {
            self.draw_object_3d(draw_state, child);
        }
    }
}

trait DrawMesh<'a> {
    fn draw_mesh(&mut self, draw_state: &mut DrawState, mesh: &'a Mesh);
}

impl<'a> DrawMesh<'a> for RenderPass<'a> {
    fn draw_mesh(&mut self, draw_state: &mut DrawState, mesh: &'a Mesh) {
        if mesh.material.is_of_type(&draw_state.current_material) {
            self.bind_material(&mesh.material);
            self.set_vertex_buffer(0, mesh.vertex_buffer.as_ref().unwrap().slice(..));
            self.set_index_buffer(
                mesh.index_buffer.as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint32,
            );
            self.draw_indexed(
                0..((std::mem::size_of_val(&*(mesh.index_array)) / std::mem::size_of::<u32>())
                    as u32),
                0,
                0..1,
            );
        }
    }
}
