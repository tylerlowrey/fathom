pub mod mesh;
pub mod camera;
mod pipeline;
pub mod vertex;

use std::f32::consts::PI;
use bevy::asset::{handle_internal_asset_events, LoadState};
use bevy::prelude::*;
use bevy::utils::HashMap;
use wgpu::{CompositeAlphaMode, InstanceDescriptor, StoreOp};
use log::{error};
use crate::app::WindowState;
use crate::assets::shaders::{Shader, ShadersState, DEFAULT_2D_SHADER, DEFAULT_3D_SHADER};
use crate::assets::tick_task_pools;
use crate::renderer::camera::Camera;
use crate::renderer::mesh::{GpuMeshes, Mesh, Mesh2D};
use crate::renderer::pipeline::Pipelines;
use crate::renderer::vertex::{Vertex, Vertex2D};

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
    mut world: &mut World,
) {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let default_3d_shader: Handle<Shader> = asset_server.load(DEFAULT_3D_SHADER);
    let default_2d_shader: Handle<Shader>= asset_server.load(DEFAULT_2D_SHADER);

    let load_state = asset_server.get_load_state(&default_3d_shader);
    log::debug!("**** Load state is: {:?}", load_state);

    tick_task_pools();

    let load_state = asset_server.get_load_state(&default_3d_shader);
    log::debug!("**** Load state is: {:?}", load_state);

    handle_internal_asset_events(world);

    let asset_server = world.get_resource::<AssetServer>().unwrap();
    loop {
        if let Some(load_state) = asset_server.get_load_state(&default_3d_shader){
            match load_state {
                LoadState::Loading => {
                    log::debug!("**** Loading shader...");
                    tick_task_pools()
                },
                LoadState::Loaded => break,
                _ => log::debug!("**** Load state is: {:?}", load_state)
            }
        }
    }

    let new_handle = default_3d_shader.clone();

    if default_3d_shader == new_handle {
        log::debug!("Cloned handles are the same {:?} == {:?}", default_3d_shader, new_handle);
    } else {
        log::debug!("Cloned handles are NOT the same {:?} != {:?}", default_3d_shader, new_handle);
    }

    world.insert_resource(Pipelines {
        registered_pipelines: HashMap::new(),
        shader_to_pipeline_id_map: HashMap::new(),
        render_pipeline_state: HashMap::new(),
    });

    let mut shader_state = ShadersState {
        loaded_shader_modules: HashMap::new(),
        shader_handles: Vec::new(),
    };

    shader_state.shader_handles.push(default_3d_shader);
    shader_state.shader_handles.push(default_2d_shader);

    world.insert_resource(shader_state);
    world.insert_resource(GpuMeshes {
        buffers_map: HashMap::new(),
    });

}

pub fn add_default_render_resources(
    renderer_state: Res<RendererState>,
    mut shaders_state: ResMut<ShadersState>,
    mut pipelines: ResMut<Pipelines>,
    shader_assets: Res<Assets<Shader>>,
) {
    tick_task_pools();
    let device = &renderer_state.as_ref().device;
    let shader_handle = shaders_state.shader_handles.get(0)
        .expect("No shader handles available. Default shader should be the first element")
        .clone();
    let default_shader = shader_assets.get(&shader_handle.clone())
        .expect("Could not get default shader from provided handle");

    log::debug!("*** DEFAULT SHADER START");
    log::debug!("================");
    log::debug!("\n{}\n", default_shader.shader_content.clone());
    log::debug!("================");
    log::debug!("*** DEFAULT SHADER END");

    let shader_module = device.create_shader_module(
        wgpu::ShaderModuleDescriptor {
            label: Some("default_3d_shader"),
            source: wgpu::ShaderSource::Wgsl(default_shader.shader_content.clone().into()) ,
        }
    );

    shaders_state.loaded_shader_modules.insert(shader_handle.clone(), shader_module);

    if let (Some(vertex_shader_module), Some(fragment_shader_module)) =
        (shaders_state.loaded_shader_modules.get(&shader_handle.clone()), shaders_state.loaded_shader_modules.get(&shader_handle.clone())) {
        let format = renderer_state.config.format.clone();

        let (pipeline_layout, uniform_buffer, uniform_bind_group) =
            Pipelines::create_uniform(device, &[Mat4::IDENTITY]);

        let render_pipeline = Pipelines::pipeline_builder(device)
            .with_label("Default 3D Render Pipeline")
            .with_layout(&pipeline_layout)
            .with_vertex_shader(&vertex_shader_module)
            .with_fragment_shader(&vertex_shader_module)
            .with_vertex_entry_point("vertex_main")
            .with_fragment_entry_point("fragment_main")
            .with_vertex_buffers(&[Vertex::vertex_buf_layout()])
            .with_color_state_targets(&[Some(format.into())])
            .build();

        pipelines.registered_pipelines.insert(1, render_pipeline);
        pipelines.shader_to_pipeline_id_map.insert(
            shader_handle.clone(),
            1
        );
        pipelines.render_pipeline_state.insert(1, (pipeline_layout, uniform_buffer, uniform_bind_group));

    } else {
        error!("Unable to create default pipeline because default shaders were not loaded");
    }
}

pub fn add_default_2d_render_resources(
    renderer_state: Res<RendererState>,
    mut shaders_state: ResMut<ShadersState>,
    mut pipelines: ResMut<Pipelines>,
    mut shader_assets: ResMut<Assets<Shader>>,
) {
    let device = &renderer_state.as_ref().device;
    let shader_handle = shaders_state.shader_handles.get(1)
        .expect("No shader handles available. Default shader should be the first element")
        .clone();
    let default_shader = shader_assets.get(&shader_handle.clone())
        .expect("Could not get default shader from provided handle");
    let shader_module = device.create_shader_module(
        wgpu::ShaderModuleDescriptor {
            label: Some("default_2d_shader"),
            source: wgpu::ShaderSource::Wgsl(default_shader.shader_content.clone().into()) ,
        }
    );

    shaders_state.loaded_shader_modules.insert(shader_handle.clone(), shader_module);

    if let Some(shader_module) = shaders_state.loaded_shader_modules.get(&shader_handle.clone()) {
        let format = renderer_state.config.format.clone();

        let render_pipeline = Pipelines::pipeline_builder(device)
            .with_label("Default 2D Render Pipeline")
            .with_vertex_shader(&shader_module)
            .with_fragment_shader(&shader_module)
            .with_vertex_entry_point("vertex_main")
            .with_fragment_entry_point("fragment_main")
            .with_vertex_buffers(&[Vertex2D::vertex_buf_layout()])
            .with_color_state_targets(&[Some(format.into())])
            .build();

        pipelines.registered_pipelines.insert(2, render_pipeline);
        pipelines.shader_to_pipeline_id_map.insert(
            shader_handle.clone(),
            2
        );
    } else {
        error!("Unable to create default pipeline because default shaders were not loaded");
    }
}

pub fn pre_render(
    window_state: ResMut<WindowState>,
    renderer_state: ResMut<RendererState>,
    pipelines: ResMut<Pipelines>,
    camera: Query<&Camera>,
) {
    let (pipeline_layout, uniform_buffer, uniform_bind_group) = pipelines.render_pipeline_state.get(&1).unwrap();

    let camera = camera.single();
    let aspect_ratio = window_state.window().inner_size().width as f32 / window_state.window().inner_size().height as f32;
    let mvp_matrix = Mat4::perspective_rh(2.0*PI/5.0, aspect_ratio, 0.1, 100.0) * camera.transform.inverse();

    renderer_state.queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[mvp_matrix]));
}

pub fn render2d(
    renderable_entities: Query<(&Renderable, &Mesh2D)>,
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
                .get(&pipelines.get_pipeline_id(&mesh.vertex_shader_handle));
            let vertex_buffer_opt = gpu_meshes.buffers_map.get(&mesh.vertex_buffer_id);


            if let (Some(pipeline), Some(vertex_buffer)) = (pipeline_opt, vertex_buffer_opt) {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                render_pass.set_pipeline(pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                if let Some(Some(index_buffer)) = &mesh.has_indices().then(|| gpu_meshes.buffers_map.get(&mesh.index_buffer_id)) {
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..mesh.num_indices() as u32, 0, 0..1);
                } else {
                    render_pass.draw(0..mesh.num_vertices() as u32, 0..1);
                }
            } else {
                error!("Unable to perform render pass for pipeline_id={} | vertex_buffer_id={}", renderable.pipline_id, mesh.vertex_buffer_id);
            }
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }

}

pub fn render3d(
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
            let pipeline_id = pipelines.get_pipeline_id(&mesh.vertex_shader_handle);
            let pipeline_opt = &pipelines.registered_pipelines
                .get(&pipeline_id);
            let vertex_buffer_opt = gpu_meshes.buffers_map.get(&mesh.vertex_buffer_id);


            if let (Some(pipeline), Some(vertex_buffer)) = (pipeline_opt, vertex_buffer_opt) {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                render_pass.set_pipeline(pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                if let Some((_, _, uniform_bind_group)) = pipelines.render_pipeline_state.get(&pipeline_id) {
                    render_pass.set_bind_group(0, &uniform_bind_group, &[]);
                }

                if let Some(Some(index_buffer)) = &mesh.has_indices().then(|| gpu_meshes.buffers_map.get(&mesh.index_buffer_id)) {
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..mesh.num_indices() as u32, 0, 0..1);
                } else {
                    render_pass.draw(0..mesh.num_vertices() as u32, 0..1);
                }
            } else {
                error!("Unable to perform render pass for pipeline_id={} | vertex_buffer_id={}", renderable.pipline_id, mesh.vertex_buffer_id);
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



/// Each entity that will be rendered must have this component
#[derive(Component, Default)]
pub struct Renderable {
    pipline_id: u64,
}

