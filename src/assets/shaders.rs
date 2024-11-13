use bevy::asset::{AssetLoader, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::{Asset, Handle, Resource, TypePath};
use bevy::utils::HashMap;
use thiserror::Error;

pub const DEFAULT_3D_SHADER: &'static str = "shaders/default.wgsl";
pub const DEFAULT_2D_SHADER: &'static str = "shaders/default_2d.wgsl";

pub type ShaderName = String;
pub type ShaderPath = String;
#[derive(Resource)]
pub struct ShadersState {
    pub(crate) loaded_shader_modules: HashMap<Handle<Shader>, wgpu::ShaderModule>,
    pub shader_handles: Vec<Handle<Shader>>,
}

#[derive(Asset, TypePath)]
pub struct Shader {
    pub(crate) shader_content: String,
}

#[derive(Default)]
pub struct ShaderAssetLoader;

#[derive(Debug, Error)]
pub enum ShaderAssetLoaderError {
    #[error("Error occurred while loading shader")]
    CouldNotLoad
}

impl AssetLoader for ShaderAssetLoader {
    type Asset = Shader;
    type Settings = ();
    type Error = ShaderAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        log::debug!("Loading shader using ShaderAssetLoader, asset path={:?}", load_context.path());
        let mut bytes = Vec::new();
        if let Ok(_) = reader.read_to_end(&mut bytes).await {
            let custom_asset = Shader {
                shader_content: String::from_utf8(bytes).unwrap(),
            };
            return Ok(custom_asset)
        }

        Err(ShaderAssetLoaderError::CouldNotLoad)
    }

    fn extensions(&self) -> &[&str] {
        &["wgsl"]
    }
}