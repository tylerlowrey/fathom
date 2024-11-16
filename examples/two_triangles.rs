use bevy::prelude::{Commands, Handle, ResMut, AssetServer};
use fathom::app::{schedule, FathomApplication, WinitApplicationState};
use fathom::assets::shaders::{Shader};
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

    commands.spawn(Mesh2D::new(
        shader_handle.clone(),
        shader_handle.clone(),
        vec![
            Vertex2D { position: [0.0, 0.5], color: [1.0, 0.0, 0.0] },
            Vertex2D { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
            Vertex2D { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
        ]
    ));

    commands.spawn(Mesh2D::new(
        shader_handle.clone(),
        shader_handle.clone(),
        vec![
            Vertex2D { position: [0.0, 0.75], color: [0.0, 1.0, 0.0] },
            Vertex2D { position: [-0.75, 0.75], color: [0.0, 1.0, 0.0] },
            Vertex2D { position: [-0.75, 0.0], color: [0.0, 1.0, 0.0] },
        ]
    ));
}