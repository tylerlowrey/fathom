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
    commands.spawn(Mesh::new(
        Shaders::default_shader_name(),
        Shaders::default_shader_name(),
        vec![
            Vertex { position: [0.0, 0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
        ]
    ));

    commands.spawn(Mesh::new(
        Shaders::default_shader_name(),
        Shaders::default_shader_name(),
        vec![
            Vertex { position: [0.0, 0.75], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-0.75, 0.75], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-0.75, 0.0], color: [0.0, 1.0, 0.0] },
        ]
    ));
}