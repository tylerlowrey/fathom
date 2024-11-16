use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Commands, ResMut};
use fathom::app::{schedule, FathomApplication};
use fathom::assets::shaders::Shader;
use fathom::renderer::mesh::Mesh2D;
use fathom::renderer::vertex::Vertex2D;

fn main() {
    let mut app = FathomApplication::with_2d_renderer();

    app.add_systems(schedule::Startup, startup);

    let _ = app.run();
}

fn startup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>
) {
    let shader_handle: Handle<Shader> = asset_server.load("shaders/default_2d.wgsl");

    commands.spawn(Mesh2D::with_indices(
        shader_handle.clone(),
        shader_handle.clone(),
        vec![
            Vertex2D { position: [0.5, 0.5], color: [0.0, 0.0, 1.0] },
            Vertex2D { position: [-0.5, 0.5], color: [0.0, 0.0, 1.0] },
            Vertex2D { position: [-0.5, -0.5], color: [0.0, 0.0, 1.0] },
            Vertex2D { position: [0.5, -0.5], color: [0.0, 0.0, 1.0] },
        ],
        vec![0, 1, 2, 0, 2, 3]
    ));

}