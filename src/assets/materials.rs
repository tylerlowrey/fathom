use bevy::prelude::*;
use crate::assets::shaders::Shader;
use crate::renderer::pipeline::PipelineId;

#[derive(Asset, TypePath)]
pub struct Material {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
    pub material_pipeline_id: PipelineId
}