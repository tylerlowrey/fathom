pub mod mesh;
mod pipeline;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bytemuck::{Pod, Zeroable};
use wgpu::{CompositeAlphaMode, InstanceDescriptor, StoreOp};
use crate::app::WindowState;
use crate::renderer::mesh::{GpuMeshes, Mesh};
use crate::renderer::pipeline::Pipelines;

pub fn initialize_renderer(mut commands: Commands, window_state: ResMut<WindowState>) {
    let window = window_state.clone_window();
    let instance = wgpu::Instance::new(InstanceDescriptor::default());
    let surface = instance.create_surface(window.clone())
        .expect("Could not create a surface");
    let adapter = pollster::block_on(create_adapter(&instance, &surface))
        .expect("Could not create wgpu adapter");
    let (device, queue) = pollster::block_on(create_device_and_queue(&adapter))
        .expect("Could not create device and queue with given adapter");
    let capabilities = surface.get_capabilities(&adapter);
    let format = capabilities.formats.iter()
        .find(|format| format.is_srgb())
        .copied()
        .expect("Unable to find a suitable texture format");
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::AutoVsync,
        desired_maximum_frame_latency: 2,
        view_formats: vec![],
        alpha_mode: CompositeAlphaMode::Auto
    };
    surface.configure(&device, &config);

    commands.insert_resource(RendererState {
        instance,
        config,
        surface,
        adapter,
        device,
        queue,
    });


}

pub fn initialize_render_resources(
    mut commands: Commands,
) {
    commands.insert_resource(Pipelines {
        registered_pipelines: HashMap::new(),
        shader_to_pipeline_id_map: HashMap::new(),
    });
    commands.insert_resource(Shaders {
        loaded_shaders: HashMap::new(),
    })
}

pub fn add_default_render_resources(
    renderer_state: Res<RendererState>,
    mut shaders: ResMut<Shaders>
) {
    let device = &renderer_state.as_ref().device;
    let shader = device.create_shader_module(
        wgpu::ShaderModuleDescriptor {
            label: Some("default_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/default.wgsl").into()) ,
        }
    );

    shaders.loaded_shaders.insert("vertex_buffers".into(), ("../../shaders/default.wgsl".into(), shader));
}

pub fn pre_render(
    mut commands: Commands,
    renderer_state: ResMut<RendererState>,
    mut shaders: ResMut<Shaders>,
    pipelines: Option<Res<Pipelines>>
) {
    let device = &renderer_state.as_ref().device;
    if pipelines.is_none() {
        let (_, shader_module) = shaders.loaded_shaders.get_mut(&Shaders::default_shader_name()).unwrap();
        let format = renderer_state.config.format.clone();
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vertex_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fragment_main",
                compilation_options: Default::default(),
                targets: &[Some(format.into())]
            }),
            multiview: None,
            cache: None,
        });

        let mut registered_pipelines = HashMap::new();
        registered_pipelines.insert(1, render_pipeline);
        let mut shader_to_pipeline_id_map = HashMap::new();
        shader_to_pipeline_id_map.insert(
            (Shaders::default_shader_name(), Shaders::default_shader_name()),
            1
        );

        commands.insert_resource(Pipelines {
            registered_pipelines,
            shader_to_pipeline_id_map,
        });
    }
}

pub fn render(
    renderable_entities: Query<(&Renderable, &Mesh)>,
    render_pipelines_opt: Option<ResMut<Pipelines>>,
    gpu_meshes: Res<GpuMeshes>,
    renderer_state: Res<RendererState>,
) {
    if let Some(pipelines) = render_pipelines_opt.as_ref() {
        let device = &renderer_state.device;
        let queue = &renderer_state.queue;
        let surface = &renderer_state.surface;
        let frame = surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main rendering command encoder")
        });
        for (renderable, mesh) in &renderable_entities {
            let pipeline_opt = &pipelines.registered_pipelines
                .get(&renderable.pipline_id);
            let vertex_buffer_opt = gpu_meshes.buffers_map.get(&mesh.gpu_buffer_id);

            if let (Some(pipeline), Some(vertex_buffer)) = (pipeline_opt, vertex_buffer_opt) {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {r: 0.05, g:0.062, b:0.08, a:1.0}),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                render_pass.set_pipeline(pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.draw(0..mesh.num_vertices() as u32, 0..1);
            } else {
                error!("Unable to perform render pass for pipeline_id={} | buffer_id={}", renderable.pipline_id, mesh.gpu_buffer_id);
            }
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }

}

async fn create_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface<'_>) -> Option<wgpu::Adapter> {
    instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: Default::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(surface),
    }).await
}

async fn create_device_and_queue(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
    adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: Default::default(),
            required_limits: Default::default(),
            memory_hints: Default::default(),
        },
        None,
    ).await
}

#[derive(Resource)]
pub struct RendererState {
    instance: wgpu::Instance,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

type ShaderName = String;
type ShaderId = u64;
type ShaderPath = String;
#[derive(Resource)]
pub struct Shaders {
    loaded_shaders: HashMap<ShaderName, (ShaderPath, wgpu::ShaderModule)>
}

impl Shaders {
    fn default_shader_name() -> String {
        "default.wgsl".into()
    }
}

/// Each entity that will be rendered must have this component
#[derive(Component, Default)]
pub struct Renderable {
    pipline_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}