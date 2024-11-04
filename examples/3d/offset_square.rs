use bevy::prelude::Commands;
use fathom::app::{schedule, GameApplication};
use fathom::assets::shaders::Shaders;
use fathom::renderer::mesh::Mesh;
use fathom::renderer::{Vertex};

fn main() {
    let mut app = GameApplication::new();

    app.add_renderer_3d();
    app.add_system_to_schedule(schedule::Startup, startup);

    let _ = app.run().unwrap();
}

fn startup(mut commands: Commands) {
    commands.spawn(Mesh::with_indices(
        Shaders::default_shader_name(),
        Shaders::default_shader_name(),
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