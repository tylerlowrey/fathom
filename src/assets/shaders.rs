use bevy::prelude::{Asset, Handle, Resource, TypePath};
use bevy::utils::HashMap;

pub const DEFAULT_3D_SHADER: &'static str = "shaders/default.wgsl";
pub const DEFAULT_2D_SHADER: &'static str = "shaders/default_2d.wgsl";

pub type ShaderName = String;
pub type ShaderPath = String;
#[derive(Resource)]
pub struct ShadersState {
    pub(crate) loaded_shader_modules: HashMap<Handle<Shader>, wgpu::ShaderModule>,
    pub shader_handles: Vec<Handle<Shader>>,
}

impl ShadersState {
    pub fn default_2d_shader_id() -> u64 {
        2
    }

    pub fn default_shader_id() -> u64 {
        1
    }
}

#[derive(Asset, TypePath)]
pub struct Shader {
    pub(crate) shader_content: String,
}
