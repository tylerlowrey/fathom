use bevy::prelude::Resource;
use bevy::utils::HashMap;
use rand::random;
use crate::assets::{AssetId, AssetType, Assets};

pub type ShaderName = String;
pub type ShaderPath = String;
#[derive(Resource)]
pub struct Shaders {
    pub(crate) loaded_shaders: HashMap<AssetId, wgpu::ShaderModule>
}

impl Shaders {
    pub fn default_2d_shader_name() -> String {
        "default_2d.wgsl".into()
    }

    pub fn default_shader_name() -> String {
        "default_2d.wgsl".into()
    }
}

pub struct Shader {
    shader_content: String,
}

impl Assets {
    fn load_shader(&mut self, path: &str) -> AssetId {
        // TODO: Return a result or fail in a useful way instead of unwrapping
        let shader_content = std::fs::read_to_string(path).unwrap();
        let id = random();
        self.loaded_assets.insert(id, AssetType::Shader(Shader {
            shader_content,
        }));

        id
    }

    fn get_shader(&self, id: &AssetId) -> Option<&Shader> {
        if let Some(AssetType::Shader(shader)) = self.loaded_assets.get(id) {
            Some(shader)
        } else {
            None
        }
    }
}