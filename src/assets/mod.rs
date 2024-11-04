use bevy::utils::HashMap;
use crate::assets::shaders::Shader;

pub mod shaders;

pub type AssetId = u64;

fn generate_asset_id() -> AssetId {
    rand::random()
}

pub enum AssetType {
    Shader(Shader)
}

pub struct Assets {
    pub loaded_assets: HashMap<AssetId, AssetType>
}