use std::num::NonZeroU64;

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BufferBinding, BufferDescriptor, BufferUsages, Device, ShaderStages,
};

use crate::renderer::wgpu_handles::WgpuHandles;

use super::{
    material::MaterialManager, staging_belt_and_command_encoder::StagingBeltAndCommandEncoder,
};

pub enum Light {
    PointLight(PointLight),
}

impl Light {
    pub fn point_light(position: glm::Vec4, color: glm::Vec3) -> Light {
        Light::PointLight(PointLight {
            data: PointLightData { position, color },
            buffer: None,
            bind_group: None,
            view_info: ViewInfo {
                view_info_buffer: None,
            },
        })
    }
}

fn set_bind_group(
    light: &mut Light,
    point_light_bind_group_layout: &Option<BindGroupLayout>,
    wgpu_handles: &WgpuHandles,
) {
    match light {
        Light::PointLight(light) => {
            if light.bind_group.is_none()
                && light.buffer.is_some()
                && light.view_info.view_info_buffer.is_some()
            {
                let _ = light
                    .bind_group
                    .insert(wgpu_handles.device.create_bind_group(&BindGroupDescriptor {
                        label: Some("SomePhongMaterialBindGroup"),
                        layout: point_light_bind_group_layout.as_ref().unwrap(),
                        entries: &[
                            BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(BufferBinding {
                                    buffer: light.buffer.as_ref().unwrap(),
                                    offset: 0,
                                    size: None,
                                }),
                            },
                            BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Buffer(BufferBinding {
                                    buffer: light.view_info.view_info_buffer.as_ref().unwrap(),
                                    offset: 0,
                                    size: None,
                                }),
                            },
                        ],
                    }));
            }
        }
    }
}

pub trait LightManager {
    fn get_point_light_bind_group_layout(&self, device: &Device) -> BindGroupLayout;
    fn write_light(
        &self,
        light: &mut Light,
        wgpu_handles: &WgpuHandles,
        staging_buffer: &mut StagingBeltAndCommandEncoder,
    );
    fn write_view_info(
        &self,
        light: &mut Light,
        view_info: &ViewInfoData,
        wgpu_handles: &WgpuHandles,
        staging_buffer: &mut StagingBeltAndCommandEncoder,
    );
}

impl LightManager for MaterialManager {
    fn get_point_light_bind_group_layout(&self, device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("PointLightBindGroupLayout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(8 * 4),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(4 * 4),
                    },
                    count: None,
                },
            ],
        })
    }

    fn write_light(
        &self,
        light: &mut Light,
        wgpu_handles: &WgpuHandles,
        staging_buffer: &mut StagingBeltAndCommandEncoder,
    ) {
        match light {
            Light::PointLight(light) => {
                let buffer = light.buffer.get_or_insert_with(|| {
                    wgpu_handles.device.create_buffer(&BufferDescriptor {
                        label: Some("SomePointLightBuffer"),
                        size: 40,
                        usage: BufferUsages::UNIFORM.union(BufferUsages::COPY_DST),
                        mapped_at_creation: false,
                    })
                });
                staging_buffer.write_buffer(
                    buffer,
                    0,
                    bytemuck::cast_slice(&[light.data]),
                    &wgpu_handles.device,
                );
            }
        }

        set_bind_group(light, &self.point_light_bind_group_layout, wgpu_handles);
    }

    fn write_view_info(
        &self,
        light: &mut Light,
        view_info: &ViewInfoData,
        wgpu_handles: &WgpuHandles,
        staging_buffer: &mut StagingBeltAndCommandEncoder,
    ) {
        match light {
            Light::PointLight(light) => {
                let buffer = light.view_info.view_info_buffer.get_or_insert_with(|| {
                    wgpu_handles.device.create_buffer(&BufferDescriptor {
                        label: Some("SomeViewInfoBuffer"),
                        size: 16,
                        usage: BufferUsages::UNIFORM.union(BufferUsages::COPY_DST),
                        mapped_at_creation: false,
                    })
                });
                staging_buffer.write_buffer(
                    buffer,
                    0,
                    bytemuck::cast_slice(&[*view_info]),
                    &wgpu_handles.device,
                );
            }
        }

        set_bind_group(light, &self.point_light_bind_group_layout, wgpu_handles);
    }
}

pub struct ViewInfo {
    pub view_info_buffer: Option<wgpu::Buffer>,
}

pub struct PointLight {
    pub data: PointLightData,
    pub buffer: Option<wgpu::Buffer>,
    pub bind_group: Option<wgpu::BindGroup>,
    pub view_info: ViewInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PointLightData {
    pub position: glm::Vec4,
    pub color: glm::Vec3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewInfoData {
    pub position: glm::Vec4,
}

unsafe impl bytemuck::Zeroable for PointLightData {
    fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

unsafe impl bytemuck::Pod for PointLightData {}

unsafe impl bytemuck::Zeroable for ViewInfoData {
    fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

unsafe impl bytemuck::Pod for ViewInfoData {}
