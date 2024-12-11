use bevy::prelude::*;
use bevy::utils::HashMap;
use bytemuck::NoUninit;
use wgpu::util::DeviceExt;
use crate::assets::materials::Material;

pub type PipelineId = u64;
#[derive(Resource)]
pub struct Pipelines {
    pub(crate) registered_pipelines: HashMap<PipelineId, wgpu::RenderPipeline>,
    pub(crate) material_to_pipeline_id_map: HashMap<Handle<Material>, PipelineId>,
    pub(crate) render_pipeline_state: HashMap<PipelineId, (wgpu::PipelineLayout, wgpu::Buffer, wgpu::BindGroup)>,
    pub default_material: Option<Handle<Material>>
}

impl Pipelines {
    pub fn get_pipeline_id_by_material(&self, material: Handle<Material>) -> Option<&PipelineId> {
         self.material_to_pipeline_id_map.get(&material)
    }
    pub fn get_pipeline_by_material(&self, material: Handle<Material>) -> Option<&wgpu::RenderPipeline> {
        if let Some(pipeline_id) = self.material_to_pipeline_id_map.get(&material) {
            return self.registered_pipelines.get(pipeline_id)
        }
        None
    }

    pub fn pipeline_builder(device: &wgpu::Device) -> PipelineBuilder {
        PipelineBuilder {
            device,
            label: None,
            layout: None,
            module: None,
            vertex_entry_point: None,
            fragment_entry_point: None,
            vertex_buffers: None,
            targets: None,
        }
    }

    pub fn create_uniform<A: NoUninit>(device: &wgpu::Device, contents: &[A]) -> (wgpu::PipelineLayout, wgpu::Buffer, wgpu::BindGroup) {
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(contents),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        (pipeline_layout, uniform_buffer, uniform_bind_group)
    }
}

pub struct PipelineBuilder<'a> {
    device: &'a wgpu::Device,
    label: wgpu::Label<'a>,
    layout: Option<&'a wgpu::PipelineLayout>,
    module: Option<&'a wgpu::ShaderModule>,
    vertex_entry_point: Option<&'a str>,
    fragment_entry_point: Option<&'a str>,
    vertex_buffers: Option<&'a [wgpu::VertexBufferLayout<'a>]>,
    targets: Option<&'a [Option<wgpu::ColorTargetState>]>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_layout(mut self, layout: &'a wgpu::PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }
    pub fn with_vertex_shader(mut self, shader_module: &'a wgpu::ShaderModule) -> Self {
        self.module = Some(shader_module);
        self
    }

    pub fn with_fragment_shader(mut self, shader_module: &'a wgpu::ShaderModule) -> Self {
        self.module = Some(shader_module);
        self
    }

    pub fn with_vertex_entry_point(mut self, vertex_entry_point: &'a str) -> Self {
        self.vertex_entry_point = Some(vertex_entry_point);
        self
    }

    pub fn with_fragment_entry_point(mut self, fragment_entry_point: &'a str) -> Self {
        self.fragment_entry_point = Some(fragment_entry_point);
        self
    }

    pub fn with_vertex_buffers(mut self, vertex_buffers: &'a [wgpu::VertexBufferLayout]) -> Self {
        self.vertex_buffers = Some(vertex_buffers);
        self
    }

    pub fn with_color_state_targets(mut self, color_state_targets: &'a [Option<wgpu::ColorTargetState>]) -> Self {
        self.targets = Some(color_state_targets);
        self
    }

    fn build_fragment_state(self) -> Option<wgpu::FragmentState<'a>> {
        if self.module.is_some() || self.fragment_entry_point.is_some() || self.targets.is_some() {
            return Some(wgpu::FragmentState {
                module: self.module.unwrap_or_else(|| panic!("Shader module is required to create a Render Pipeline")),
                entry_point: self.fragment_entry_point.unwrap_or_else(|| panic!("Fragment entry point is required to create a Render Pipeline")),
                compilation_options: Default::default(),
                targets: self.targets.unwrap_or(&[]),
            })
        }

        None
    }
    pub fn build(self) -> wgpu::RenderPipeline {
        self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label,
            layout: self.layout,
            vertex: wgpu::VertexState {
                module: self.module.unwrap_or_else(|| panic!("Shader module is required to create a Render Pipeline")),
                entry_point:  self.vertex_entry_point.unwrap_or_else(|| panic!("Vertex entry point is required to create a Render Pipeline")),
                compilation_options: Default::default(),
                buffers: self.vertex_buffers.unwrap_or(&[]),
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            fragment: self.build_fragment_state(),
            multiview: None,
            cache: None,
        })
    }
}

