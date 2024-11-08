use bevy::asset::AssetServerMode;
use bevy::asset::io::{AssetSourceBuilders};
use bevy::prelude::*;
use crate::assets::shaders::Shader;

const DEFAULT_ASSETS_PATH: &str = "assets";

pub mod shaders;

pub fn initialize_asset_server(mut commands: Commands) {
    let mut builders: AssetSourceBuilders = Default::default();
    builders.init_default_source(DEFAULT_ASSETS_PATH, None);
    let asset_sources = builders.build_sources(false, false);
    let asset_server = AssetServer::new(
        asset_sources,
        AssetServerMode::Unprocessed,
        false
    );

    let shader_assets = Assets::<Shader>::default();
    asset_server.register_asset(&shader_assets);

    commands.insert_resource(shader_assets);
    commands.insert_resource(asset_server);
}