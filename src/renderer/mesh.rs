use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bytemuck::cast_slice;
use rand::random;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::renderer::{Renderable, RendererState, Vertex};

#[derive(Component)]
#[component(on_add = create_gpu_buffer_for_mesh)]
#[require(Renderable)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    pub vertex_shader_name: String,
    pub fragment_shader_name: String,
    pub gpu_buffer_id: u64,
}

impl Mesh {
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
}

#[derive(Resource)]
pub struct GpuMeshes {
    pub buffers_map: HashMap<u64, wgpu::Buffer>
}


pub fn create_gpu_buffer_for_mesh(
    mut world: DeferredWorld, entity: Entity, _component_id: ComponentId
) {
    let device = &world.get_resource::<RendererState>().unwrap().device;
    let mesh_component = world.get::<Mesh>(entity).unwrap();
    let buffer_id = random();
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some(format!("{}", buffer_id).as_str()),
        contents: cast_slice(&mesh_component.vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    info!("Creating gpu buffer for mesh with shaders vertex={} | fragment={}",
        &mesh_component.vertex_shader_name, mesh_component.fragment_shader_name);

    let mut gpu_meshes = world.get_resource_mut::<GpuMeshes>().unwrap();

    gpu_meshes.buffers_map.insert(buffer_id, buffer);
    let mut mesh_component = world.get_mut::<Mesh>(entity).unwrap();
    mesh_component.gpu_buffer_id = buffer_id;

    info!("Created gpu buffer for entity_id={} | vertex_shader_name={} | fragment_shader_name={} \
    | gpu_buffer_id={}", entity, mesh_component.vertex_shader_name, mesh_component.fragment_shader_name, buffer_id);
}