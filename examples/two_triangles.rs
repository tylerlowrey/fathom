use bevy::prelude::Commands;
use fathom::app::{schedule, GameApplication};
use fathom::assets::shaders::ShadersState;
use fathom::renderer::mesh::Mesh2D;
use fathom::renderer::{Vertex2D};

fn main() {
    let mut app = GameApplication::new();

    app.add_renderer_2d();
    app.add_system_to_schedule(schedule::Startup, startup);

    let _ = app.run().unwrap();
}

fn startup(mut commands: Commands) {
    commands.spawn(Mesh2D::new(
        ShadersState::default_2d_shader_id(),
        ShadersState::default_2d_shader_id(),
        vec![
            Vertex2D { position: [0.0, 0.5], color: [1.0, 0.0, 0.0] },
            Vertex2D { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
            Vertex2D { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
        ]
    ));

    commands.spawn(Mesh2D::new(
        ShadersState::default_2d_shader_id(),
        ShadersState::default_2d_shader_id(),
        vec![
            Vertex2D { position: [0.0, 0.75], color: [0.0, 1.0, 0.0] },
            Vertex2D { position: [-0.75, 0.75], color: [0.0, 1.0, 0.0] },
            Vertex2D { position: [-0.75, 0.0], color: [0.0, 1.0, 0.0] },
        ]
    ));
}