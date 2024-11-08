use bevy::prelude::*;
use log::{error, info};
use fathom::app::{schedule, GameApplication};
use fathom::assets::shaders::ShadersState;
use fathom::renderer::mesh::{Mesh, Mesh2D};
use fathom::renderer::{Vertex2D};

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    let mut app = GameApplication::new();

    app.add_renderer_3d();
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