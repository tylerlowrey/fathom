use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bytemuck::cast_slice;
use rand::random;
use log::{info, error};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::renderer::{Renderable, RendererState, Vertex};
use crate::renderer::pipeline::Pipelines;

#[derive(Component)]
#[require(Renderable)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Option<Vec<u32>>,
    pub vertex_shader_name: String,
    pub fragment_shader_name: String,
    pub vertex_buffer_id: u64,
    pub index_buffer_id: u64,
}

impl Mesh {
    pub fn new(
        vertex_shader_name: String,
        fragment_shader_name: String,
        vertices: Vec<Vertex>
    ) -> Self {
        Self {
            vertices,
            indices: None,
            vertex_shader_name,
            fragment_shader_name,
            vertex_buffer_id: 0,
            index_buffer_id: 0,
        }
    }

    pub fn with_indices(
        vertex_shader_name: String,
        fragment_shader_name: String,
        vertices: Vec<Vertex>,
        indices: Vec<u32>
    ) -> Self {
        Self {
            vertices,
            indices: Some(indices),
            vertex_shader_name,
            fragment_shader_name,
            vertex_buffer_id: 0,
            index_buffer_id: 0,
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_indices(&self) -> usize {
        if let Some(indices) = &self.indices {
            indices.len()
        } else {
            0
        }
    }

    pub fn has_indices(&self) -> bool {
        self.indices.is_some()
    }
}

pub fn setup_on_add_hook_for_mesh(world: &mut World) {
    world.register_component_hooks::<Mesh>().on_add(create_gpu_buffer_for_mesh);
}

#[derive(Resource)]
pub struct GpuMeshes {
    pub buffers_map: HashMap<u64, wgpu::Buffer>
}


pub fn create_gpu_buffer_for_mesh(
    mut world: DeferredWorld, entity: Entity, _component_id: ComponentId
) {
    let mesh_component = world.get::<Mesh>(entity).unwrap();
    let vertex_shader_name = mesh_component.vertex_shader_name.clone();
    let fragment_shader_name = mesh_component.fragment_shader_name.clone();

    if mesh_component.vertex_buffer_id == 0 {
        let device = &world.get_resource::<RendererState>().unwrap().device;
        let buffer_id = random();
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(format!("{}", buffer_id).as_str()),
            contents: cast_slice(&mesh_component.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        info!("Creating vertex buffer for mesh with shaders vertex={} | fragment={}",
            mesh_component.vertex_shader_name, mesh_component.fragment_shader_name);

        let mut gpu_meshes = world.get_resource_mut::<GpuMeshes>().unwrap();

        gpu_meshes.buffers_map.insert(buffer_id, buffer);
        let mut mesh_component = world.get_mut::<Mesh>(entity).unwrap();
        mesh_component.vertex_buffer_id = buffer_id;

        info!("Created vertex gpu buffer for entity_id={} | vertex_shader_name={} | fragment_shader_name={} \
            | gpu_buffer_id={}", entity, mesh_component.vertex_shader_name, mesh_component.fragment_shader_name, buffer_id);
    }

    let mesh_component = world.get::<Mesh>(entity).unwrap();
    if let Some(indices) = &mesh_component.indices {
        if mesh_component.index_buffer_id == 0 {
            let device = &world.get_resource::<RendererState>().unwrap().device;
            let buffer_id = random();
            let buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some(format!("{}", buffer_id).as_str()),
                contents: cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            info!("Creating index buffer for mesh with shaders vertex={} | fragment={}",
                mesh_component.vertex_shader_name, mesh_component.fragment_shader_name);

            let mut gpu_meshes = world.get_resource_mut::<GpuMeshes>().unwrap();

            gpu_meshes.buffers_map.insert(buffer_id, buffer);
            let mut mesh_component = world.get_mut::<Mesh>(entity).unwrap();
            mesh_component.index_buffer_id = buffer_id;

            info!("Created index gpu buffer for entity_id={} | vertex_shader_name={} | fragment_shader_name={} \
            | gpu_buffer_id={}", entity, mesh_component.vertex_shader_name, mesh_component.fragment_shader_name, buffer_id);
        }
    }

    let renderable_component = world.get::<Renderable>(entity).unwrap();
    if renderable_component.pipline_id == 0 {
        let pipelines = world.get_resource::<Pipelines>().unwrap();
        let pipeline_id_opt = pipelines.shader_to_pipeline_id_map.get(&(vertex_shader_name, fragment_shader_name));
        if let Some(pipeline_id) = pipeline_id_opt {
            let pipeline_id = *pipeline_id;
            let mut renderable_component = world.get_mut::<Renderable>(entity).unwrap();
            renderable_component.pipline_id = pipeline_id;
        } else {
            error!("Pipeline id not found for Renderable");
        }
    }
}