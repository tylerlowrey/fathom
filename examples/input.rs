use bevy::prelude::{AppExit, EventReader};
use winit::event::ElementState;
use winit::keyboard::Key;
use fathom::app::{schedule, FathomApplication};
use fathom::input::InputEvent;

pub fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    let mut app = FathomApplication::new();

    app.add_systems(schedule::Update, handle_key_input);

    let app_run_result = app.run();

    if let AppExit::Error(error) = app_run_result {
        log::error!("App run failed with error code: {}", error);
    }
}

pub fn handle_key_input(
    mut input_events: EventReader<InputEvent>
) {
    for event in input_events.read() {
        match event {
            InputEvent::Keyboard(key_event) => {
                log::info!("{:?}", key_event);
            }
            _ => ()
        }
    }
}