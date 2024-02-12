use std::{collections::HashMap, num::NonZeroU64};

use glm::Vector3;
use strum_macros::EnumIter;
use wgpu::{
    include_wgsl, vertex_attr_array, BindGroup, BindGroupDescriptor, BindGroupEntry,
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Buffer, BufferBinding,
    BufferDescriptor, BufferUsages, Device, Face, FragmentState, MultisampleState,
    PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor,
    ShaderModule, ShaderStages, SurfaceCapabilities, Texture, VertexBufferLayout, VertexState,
};

use super::{
    light::LightManager, staging_belt_and_command_encoder::StagingBeltAndCommandEncoder,
    wgpu_handles::WgpuHandles,
};

pub struct MaterialManager {
    pub shaders: Vec<ShaderModule>,
    pub render_pipelines: Vec<RenderPipeline>,
    pub material_bind_group_layouts: Vec<BindGroupLayout>,
    pub matrix_bind_group_layout: Option<BindGroupLayout>,
    pub point_light_bind_group_layout: Option<BindGroupLayout>,
    pub view_info_buffer: Option<Buffer>,
    pub view_info_bind_group: Option<BindGroup>,
    pub textures: HashMap<String, Texture>,
}

#[derive(Debug)]
pub enum Material {
    PhongMaterial(PhongMaterial),
    PhongMaterialWithTexture(PhongMaterialWithTexture),
}

#[derive(Clone, EnumIter)]
pub enum MaterialType {
    PhongMaterial,
    PhongMaterialWithTexture,
}

impl Material {
    pub fn is_of_type(&self, compare_to: &MaterialType) -> bool {
        match (self, compare_to) {
            (Material::PhongMaterial(_), MaterialType::PhongMaterial) => true,
            (Material::PhongMaterialWithTexture(_), MaterialType::PhongMaterialWithTexture) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct PhongMaterial {
    pub data: PhongMaterialData,
    pub buffer: Option<Buffer>,
    pub bind_group: Option<BindGroup>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhongMaterialData {
    ka: glm::Vector3<f32>,
    _padding1: u8,
    kd: glm::Vector3<f32>,
    _padding2: u8,
    ks: glm::Vector3<f32>,
    shininess: f32,
}

unsafe impl bytemuck::Zeroable for PhongMaterialData {}
unsafe impl bytemuck::Pod for PhongMaterialData {}

#[derive(Debug)]
pub struct PhongMaterialWithTexture {
    pub data: PhongMaterialData,
    map_kd: String,
    texture_loaded: bool,
    pub buffer: Option<Buffer>,
    pub bind_group: Option<BindGroup>,
}

impl PartialEq for PhongMaterialWithTexture {
    fn eq(&self, other: &Self) -> bool {
        self.map_kd == other.map_kd
            && self.data.ka == other.data.ka
            && self.data.kd == other.data.kd
            && self.data.ks == other.data.ks
            && self.data.shininess == other.data.shininess
    }
}

impl Eq for PhongMaterialWithTexture {}

impl PhongMaterial {
    pub fn new_without_texture(
        ka: Vector3<f32>,
        kd: Vector3<f32>,
        ks: Vector3<f32>,
        shininess: f32,
    ) -> Self {
        PhongMaterial {
            data: PhongMaterialData {
                ka,
                kd,
                ks,
                shininess,
                _padding1: 0,
                _padding2: 0,
            },
            buffer: None,
            bind_group: None,
        }
    }

    pub fn new_with_texture(
        ka: Vector3<f32>,
        kd: Vector3<f32>,
        ks: Vector3<f32>,
        shininess: f32,
        map_kd: &String,
    ) -> PhongMaterialWithTexture {
        PhongMaterialWithTexture {
            data: PhongMaterialData {
                ka,
                kd,
                ks,
                shininess,
                _padding1: 0,
                _padding2: 0,
            },
            map_kd: map_kd.to_owned(),
            texture_loaded: false,
            buffer: None,
            bind_group: None,
        }
    }
}

impl MaterialManager {
    pub fn new() -> MaterialManager {
        MaterialManager {
            shaders: vec![],
            render_pipelines: vec![],
            material_bind_group_layouts: vec![],
            matrix_bind_group_layout: None,
            point_light_bind_group_layout: None,
            view_info_buffer: None,
            view_info_bind_group: None,
            textures: HashMap::new(),
        }
    }

    fn add_phong_material(&mut self, device: &Device, surface_capabilities: &SurfaceCapabilities) {
        let shader_module =
            device.create_shader_module(include_wgsl!("./shaders/phong-model.wgsl"));

        let matrix_bind_group_layout = self.matrix_bind_group_layout.as_ref().unwrap();
        let point_light_bind_group_layout = self.point_light_bind_group_layout.as_ref().unwrap();

        let material_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("PhongMaterialBindGroupLayout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(16 * 4),
                    },
                    count: None,
                }],
            });

        // Bind Groups
        // Group 0 Binding 0: Object to Camera Matrix Uniform
        // Group 1 Binding 0: PhongMaterialData
        // Group 2 Binding 0: PointLigthData
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("PhongShadingPipelineLayout"),
            bind_group_layouts: &[
                matrix_bind_group_layout,
                &material_bind_group_layout,
                point_light_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("PhongShadingRenderPipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vertex_main",
                buffers: &[VertexBufferLayout {
                    array_stride: 48,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4],
                }],
            },
            // TODO: I don't know if this will work or not.
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fragment_main",
                targets: &[Some(surface_capabilities.formats[0].into())],
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,         // not strip topology
                front_face: wgpu::FrontFace::Ccw, // opengl
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            multisample: MultisampleState::default(),
            depth_stencil: None,
            multiview: None,
        });

        self.render_pipelines.push(render_pipeline);
        self.material_bind_group_layouts
            .push(material_bind_group_layout);
    }

    fn create_common_bind_group_layouts(&mut self, device: &Device) {
        self.matrix_bind_group_layout =
            Some(device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("PhongShadingBindGroupLayout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(2 * 16 * 4),
                    },
                    count: None,
                }],
            }));

        self.point_light_bind_group_layout = Some(self.get_point_light_bind_group_layout(device));
    }

    pub fn populate(&mut self, device: &Device, surface_capabilities: &SurfaceCapabilities) {
        self.create_common_bind_group_layouts(device);
        self.add_phong_material(device, surface_capabilities);
    }

    pub fn get_pipeline_for_material_type(&self, material_type: &MaterialType) -> &RenderPipeline {
        match material_type {
            MaterialType::PhongMaterial => self.render_pipelines.get(0).unwrap(),
            MaterialType::PhongMaterialWithTexture => todo!(),
        }
    }

    pub fn write_material(
        &self,
        material: &mut Material,
        wgpu_handles: &WgpuHandles,
        staging_belt: &mut StagingBeltAndCommandEncoder,
    ) {
        match material {
            Material::PhongMaterial(mat) => {
                let buffer = mat.buffer.get_or_insert_with(|| {
                    wgpu_handles.device.create_buffer(&BufferDescriptor {
                        label: Some("SomePhongMaterialBuffer"),
                        size: 64,
                        usage: BufferUsages::UNIFORM.union(BufferUsages::COPY_DST),
                        mapped_at_creation: false,
                    })
                });
                mat.bind_group.get_or_insert_with(|| {
                    wgpu_handles.device.create_bind_group(&BindGroupDescriptor {
                        label: Some("SomePhongMaterialBindGroup"),
                        layout: self.material_bind_group_layouts.get(0).unwrap(),
                        entries: &[BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer(BufferBinding {
                                buffer,
                                offset: 0,
                                size: None,
                            }),
                        }],
                    })
                });
                staging_belt.write_buffer(
                    buffer,
                    0,
                    bytemuck::cast_slice(&[mat.data]),
                    &wgpu_handles.device,
                );
            }
            Material::PhongMaterialWithTexture(_) => todo!(),
        }
    }
}
