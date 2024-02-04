use super::material::MaterialManager;
use std::sync::Arc;

pub struct WgpuHandles {
    pub adapter: wgpu::Adapter,
    pub instance: wgpu::Instance,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub material_manager: Arc<MaterialManager>,
}
