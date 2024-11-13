use bevy::prelude::*;
use fathom::app::{schedule, GameApplication};
use fathom::assets::shaders::{Shader};
use fathom::renderer::camera::Camera;
use fathom::renderer::mesh::Mesh;
use fathom::renderer::vertex::Vertex;

fn main() {
    let mut app = GameApplication::new();

    app.add_renderer_3d();
    app.add_system_to_schedule(schedule::Startup, startup);

    let _ = app.run().unwrap();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let shader_handle: Handle<Shader> = asset_server.load("shaders/default.wgsl");
    commands.spawn(Mesh::with_indices(
        shader_handle.clone(),
        shader_handle.clone(),
        vec![
            Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0,  1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0,  1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [-1.0,  1.0,  1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0, -1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, -1.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [-1.0,  1.0, -1.0], color: [1.0, 0.0, 1.0] },
        ],
        vec![
            0, 1, 2, 2, 3, 0,
            4, 5, 6, 6, 7, 4,
            4, 0, 3, 3, 7, 4,
            1, 5, 6, 6, 2, 1,
            3, 2, 6, 6, 7, 3,
            4, 5, 1, 1, 0, 4,
        ]
    ));

    commands.spawn(Camera {
        transform: Mat4::look_at_rh(Vec3::new(5.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y).inverse()
    });

}