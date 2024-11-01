use bevy::prelude::{Commands, ResMut, Resource};
use log::{error, info};
use fathom::app::{schedule, GameApplication};
use fathom::renderer::mesh::Mesh;
use fathom::renderer::{Shaders, Vertex};

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    let mut app = GameApplication::new();

    app.add_renderer();
    app.add_system_to_schedule(schedule::Startup, startup);
    app.add_system_to_schedule(schedule::Update, update);

    let app_run_result = app.run();

    if let Err(error) = app_run_result {
        error!("App run failed: {}", error);
    }

    info!("App finished, exiting...")
}

fn startup(mut commands: Commands) {
    commands.insert_resource(Counter {
        count: 0
    });

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



fn update(mut counter: ResMut<Counter>) {
    counter.count += 1;
    if counter.count % 20000 == 0 {
        info!("Count reached: {}", counter.count);
    }
}

#[derive(Resource, Default)]
struct Counter {
    pub count: u64
}