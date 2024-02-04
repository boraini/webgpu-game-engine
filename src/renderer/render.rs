use std::sync::Arc;

use wgpu::{util::StagingBelt, CommandEncoderDescriptor, TextureView};

use super::{
    draw_impl::DrawObject3D,
    draw_state::DrawState,
    material::{MaterialManager, MaterialType},
    staging_belt_and_command_encoder::StagingBeltAndCommandEncoder,
    wgpu_handles::WgpuHandles,
};
use crate::{
    renderer::light::Light,
    scene::{object3d::Object3DManager, scene::Scene},
};

pub fn render_to_texture_view(
    scene: &mut Scene,
    wgpu_handles: &mut WgpuHandles,
    texture_view: &TextureView,
    material_manager: &mut Arc<MaterialManager>,
) {
    let encoder = wgpu_handles
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("SceneRenderCommandEncoder"),
        });
    let staging_belt = StagingBelt::new(128);
    let mut staging_belt = StagingBeltAndCommandEncoder::new(staging_belt, encoder);
    render(
        scene,
        wgpu_handles,
        texture_view,
        material_manager,
        &mut staging_belt,
    );
    staging_belt.staging_belt.finish();
    wgpu_handles
        .queue
        .submit(Some(staging_belt.command_encoder.finish()));
    staging_belt.staging_belt.recall();
}

pub fn render(
    scene: &mut Scene,
    wgpu_handles: &mut WgpuHandles,
    texture_view: &TextureView,
    material_manager: &mut Arc<MaterialManager>,
    staging_belt: &mut StagingBeltAndCommandEncoder,
) {
    let mut draw_state = DrawState::new(MaterialType::PhongMaterial, material_manager.clone());
    draw_state.transform(&scene.camera.get_inverse_matrix());

    // objects are only written once by policy
    wgpu_handles
        .material_manager
        .write_object_3d(&mut scene.root, wgpu_handles);

    // matrices and materials are written each frame
    scene
        .root
        .write_matrices(wgpu_handles, &mut draw_state, staging_belt);
    scene
        .root
        .write_materials(wgpu_handles, material_manager, staging_belt);

    scene.write_lights(wgpu_handles, material_manager, staging_belt);
    scene.write_view_info(wgpu_handles, material_manager, staging_belt);

    let mut render_pass =
        staging_belt
            .command_encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 0.6,
                            b: 0.8,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

    // only a set of iterations for point light
    for material_type in &[MaterialType::PhongMaterial] {
        draw_state.current_material = material_type.to_owned();

        render_pass.set_pipeline(material_manager.get_pipeline_for_material_type(material_type));
        for light in &scene.lights {
            match light {
                Light::PointLight(point_light) => {
                    render_pass.set_bind_group(2, point_light.bind_group.as_ref().unwrap(), &[]);
                    render_pass.draw_object_3d(&mut draw_state, &scene.root);
                }
            }
        }
    }
}
