use bevy::prelude::*;
use fathom::app::{schedule, GameApplication};
use fathom::assets::Assets;
use fathom::assets::shaders::ShadersState;
use fathom::renderer::mesh::Mesh;
use fathom::renderer::{Vertex};

fn main() {
    let mut app = GameApplication::new();

    app.add_renderer_3d();
    app.add_system_to_schedule(schedule::Startup, startup);

    let _ = app.run().unwrap();
}

fn startup(
    mut commands: Commands,
    mut assets: ResMut<Assets>
) {
    assets.load_shader("shaders/default.wgsl");
    commands.spawn(Mesh::with_indices(
        ShadersState::default_shader_id(),
        ShadersState::default_shader_id(),
        vec![
            Vertex { position: [3.0, 3.0, -1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [3.0, 1.0, -1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [3.0, 3.0, 1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [3.0, 1.0, 1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [1.0, 3.0, -1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [1.0, 1.0, -1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [1.0, 3.0, 1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [1.0, 1.0, 1.0], color: [0.0, 0.0, 1.0] },
        ],
        // TODO: Edit these with the actual values based on referenced .obj file (scene_with_square.obj)
        vec![0, 1, 2, 0, 2, 3]
    ));

}