use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::renderer::{ShaderName};

#[derive(Resource)]
pub struct Pipelines {
    pub(crate) registered_pipelines: HashMap<u64, wgpu::RenderPipeline>,
    pub(crate) shader_to_pipeline_id_map: HashMap<(ShaderName, ShaderName), u64>
}

impl Pipelines {
    pub fn get_pipeline_id(&self, vertex_shader: ShaderName, fragment_shader: ShaderName) -> u64 {
        self.shader_to_pipeline_id_map.get(&(vertex_shader, fragment_shader))
            .unwrap_or(&0u64).clone()
    }
}
