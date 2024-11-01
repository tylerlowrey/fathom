use bevy::prelude::Commands;
use fathom::app::{schedule, GameApplication};
use fathom::renderer::mesh::Mesh;
use fathom::renderer::{Shaders, Vertex};

fn main() {
    let mut app = GameApplication::new();

    app.add_renderer();
    app.add_system_to_schedule(schedule::Startup, startup);

    let _ = app.run().unwrap();
}

fn startup(mut commands: Commands) {
    commands.spawn(Mesh::with_indices(
        Shaders::default_shader_name(),
        Shaders::default_shader_name(),
        vec![
            Vertex { position: [0.5, 0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-0.5, 0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-0.5, -0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0] },
        ],
        vec![0, 1, 2, 0, 2, 3]
    ));

}