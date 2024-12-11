use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bytemuck::cast_slice;
use rand::random;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::renderer::{Renderable, RendererState, Vertex, Vertex2D};

#[derive(Component)]
#[require(Renderable)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Option<Vec<u32>>,
    pub vertex_buffer_id: u64,
    pub index_buffer_id: u64,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vertex>
    ) -> Self {
        Self {
            vertices,
            indices: None,
            vertex_buffer_id: 0,
            index_buffer_id: 0,
        }
    }

    pub fn with_indices(
        vertices: Vec<Vertex>,
        indices: Vec<u32>
    ) -> Self {
        Self {
            vertices,
            indices: Some(indices),
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

    if mesh_component.vertex_buffer_id == 0 {
        let device = &world.get_resource::<RendererState>().unwrap().device;
        let buffer_id = random();
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(format!("{}", buffer_id).as_str()),
            contents: cast_slice(&mesh_component.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        log::info!("Creating vertex buffer for mesh");

        let mut gpu_meshes = world.get_resource_mut::<GpuMeshes>().unwrap();

        gpu_meshes.buffers_map.insert(buffer_id, buffer);
        let mut mesh_component = world.get_mut::<Mesh>(entity).unwrap();
        mesh_component.vertex_buffer_id = buffer_id;

        log::info!("Created vertex gpu buffer for entity_id={} | gpu_buffer_id={}", entity, buffer_id);
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

            log::info!("Creating index buffer for mesh");

            let mut gpu_meshes = world.get_resource_mut::<GpuMeshes>().unwrap();

            gpu_meshes.buffers_map.insert(buffer_id, buffer);
            let mut mesh_component = world.get_mut::<Mesh>(entity).unwrap();
            mesh_component.index_buffer_id = buffer_id;

            log::info!("Created index gpu buffer for entity_id={} | gpu_buffer_id={}", entity, buffer_id);
        }
    }
}

#[derive(Component)]
#[require(Renderable)]
pub struct Mesh2D {
    vertices: Vec<Vertex2D>,
    indices: Option<Vec<u32>>,
    pub vertex_buffer_id: u64,
    pub index_buffer_id: u64,
}

impl Mesh2D {
    pub fn new(
        vertices: Vec<Vertex2D>
    ) -> Self {
        Self {
            vertices,
            indices: None,
            vertex_buffer_id: 0,
            index_buffer_id: 0,
        }
    }

    pub fn with_indices(
        vertices: Vec<Vertex2D>,
        indices: Vec<u32>
    ) -> Self {
        Self {
            vertices,
            indices: Some(indices),
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

pub fn setup_on_add_hook_for_mesh2d(world: &mut World) {
    world.register_component_hooks::<Mesh2D>().on_add(create_gpu_buffer_for_mesh2d);
}

pub fn create_gpu_buffer_for_mesh2d(
    mut world: DeferredWorld, entity: Entity, _component_id: ComponentId
) {
    let mesh_component = world.get::<Mesh2D>(entity).unwrap();

    if mesh_component.vertex_buffer_id == 0 {
        let device = &world.get_resource::<RendererState>().unwrap().device;
        let buffer_id = random();
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(format!("{}", buffer_id).as_str()),
            contents: cast_slice(&mesh_component.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        log::info!("Creating vertex buffer for mesh");

        let mut gpu_meshes = world.get_resource_mut::<GpuMeshes>().unwrap();

        gpu_meshes.buffers_map.insert(buffer_id, buffer);
        let mut mesh_component = world.get_mut::<Mesh2D>(entity).unwrap();
        mesh_component.vertex_buffer_id = buffer_id;

        log::info!("Created vertex gpu buffer for entity_id={} | gpu_buffer_id={}", entity, buffer_id);
    }

    let mesh_component = world.get::<Mesh2D>(entity).unwrap();
    if let Some(indices) = &mesh_component.indices {
        if mesh_component.index_buffer_id == 0 {
            let device = &world.get_resource::<RendererState>().unwrap().device;
            let buffer_id = random();
            let buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some(format!("{}", buffer_id).as_str()),
                contents: cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            log::info!("Creating index buffer for mesh2d");

            let mut gpu_meshes = world.get_resource_mut::<GpuMeshes>().unwrap();

            gpu_meshes.buffers_map.insert(buffer_id, buffer);
            let mut mesh_component = world.get_mut::<Mesh2D>(entity).unwrap();
            mesh_component.index_buffer_id = buffer_id;

            log::info!("Created index gpu buffer for entity_id={} | gpu_buffer_id={}", entity, buffer_id);
        }
    }
}
