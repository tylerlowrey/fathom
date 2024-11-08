use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::assets::shaders::Shader;

#[derive(Resource)]
pub struct Pipelines {
    pub(crate) registered_pipelines: HashMap<u64, wgpu::RenderPipeline>,
    pub(crate) shader_to_pipeline_id_map: HashMap<Handle<Shader>, u64>
}

impl Pipelines {
    pub fn get_pipeline_id(&self, shader_handle: &Handle<Shader>) -> u64 {
        self.shader_to_pipeline_id_map.get(shader_handle)
            .unwrap_or(&0u64).clone()
    }
}
