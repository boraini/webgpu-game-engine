use crate::{
    renderer::wgpu_handles::WgpuHandles,
    renderer::{
        light::{Light, LightManager},
        material::MaterialManager,
        staging_belt_and_command_encoder::StagingBeltAndCommandEncoder,
    },
    util,
};

use super::{camera::Camera, object3d::Object3D};

pub struct Scene {
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub root: Object3D,
}

impl Scene {
    pub fn new() -> Scene {
        let camera = Camera::PerspectiveCamera {
            world_to_local: util::identity_matrix(),
            near: 0.1,
            far: 3.0,
            aspect: 1.0,
        };
        Scene {
            lights: vec![],
            camera,
            root: Object3D::create_empty(),
        }
    }
    pub fn write_lights(
        &mut self,
        wgpu_handles: &WgpuHandles,
        material_manager: &MaterialManager,
        staging_buffer: &mut StagingBeltAndCommandEncoder,
    ) {
        for light in &mut self.lights {
            material_manager.write_light(light, wgpu_handles, staging_buffer);
        }
    }
    pub fn write_view_info(
        &mut self,
        wgpu_handles: &WgpuHandles,
        material_manager: &MaterialManager,
        staging_buffer: &mut StagingBeltAndCommandEncoder,
    ) {
        let view_info = &self.camera.get_view_info();
        for light in &mut self.lights {
            material_manager.write_view_info(light, view_info, wgpu_handles, staging_buffer);
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
