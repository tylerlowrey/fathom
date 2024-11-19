use std::f32::consts::PI;
use bevy::prelude::*;
use fathom::app::{schedule, FathomApplication, WinitApplicationState};
use fathom::assets::shaders::{Shader, ShadersState};
use fathom::renderer::camera::Camera;
use fathom::renderer::mesh::{Mesh, Mesh2D};
use fathom::renderer::vertex::Vertex;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Warn).init();
    let mut app = FathomApplication::with_3d_renderer();

    app.add_systems(schedule::Startup, startup);
    app.add_systems(schedule::Update, update);

    let app_run_result = app.run();

    if let AppExit::Error(error) = app_run_result {
        log::error!("App run failed with error code: {}", error);
    }

    log::info!("App finished, exiting...")
}

fn startup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>
) {
    commands.insert_resource(Counter {
        count: 0
    });

    let shader_handle: Handle<Shader> = asset_server.load("shaders/default.wgsl");

    /*
    commands.spawn(Mesh::with_indices(
        shader_handle.clone(),
        shader_handle.clone(),
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
        vec![1, 5, 7, 3, 4, 3, 7, 8, 8, 7, 5, 6, 6, 2, 4, 8, 2, 1, 3, 4, 6, 5, 1, 2]
    ));*/

    commands.spawn(Mesh::with_indices(
        shader_handle.clone(),
        shader_handle.clone(),
        vec![
            Vertex { position: [-1.0, -1.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-1.0, -1.0, -1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0, -1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] },
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
        transform: Mat4::look_at_rh(Vec3::new(10.0, 10.0, 10.0), Vec3::ZERO, Vec3::Y).inverse()
    });
}



fn update(mut counter: ResMut<Counter>, mut camera_query: Query<&mut Camera>) {
    counter.count += 1;
    if counter.count % 10 == 0 {
        log::warn!("Count reached: {}", counter.count);
        let count = (counter.count as f32 % 1000.0) / 1000.0;
        let mut camera = camera_query.single_mut();
        camera.transform = Mat4::from_rotation_y((count * (4.0 * PI)) - (2.0 * PI)) * camera.transform;
    }
}

#[derive(Resource, Default)]
struct Counter {
    pub count: u64
}