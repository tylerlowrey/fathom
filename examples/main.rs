use std::f32::consts::PI;
use bevy::prelude::*;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::PhysicalKey;
use fathom::app::{schedule, FathomApplication, WinitApplicationState};
use fathom::assets::shaders::{Shader, ShadersState};
use fathom::input::InputEvent;
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
    let shader_handle: Handle<Shader> = asset_server.load("shaders/custom_3d_shader.wgsl");

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

fn update(
    mut camera_query: Query<&mut Camera>,
    mut input_events: EventReader<InputEvent>
) {
    let mut camera = camera_query.single_mut();
    for InputEvent::Keyboard(KeyEvent { physical_key,  state, ..}) in input_events.read() {
        match (physical_key, state) {
            (PhysicalKey::Code(winit::keyboard::KeyCode::KeyW), ElementState::Pressed) => {
                camera.transform.w_axis.z -= 0.5;
            },
            (PhysicalKey::Code(winit::keyboard::KeyCode::KeyS), ElementState::Pressed) => {
                camera.transform.w_axis.z += 0.5;
            },
            (PhysicalKey::Code(winit::keyboard::KeyCode::KeyA), ElementState::Pressed) => {
                camera.transform.w_axis.x -= 0.5;
            },
            (PhysicalKey::Code(winit::keyboard::KeyCode::KeyD), ElementState::Pressed) => {
                camera.transform.w_axis.x += 0.5;
            },
            _ => ()
        }
    }
}