/// Fathom is a game engine built on top of bevy_ecs, bevy_asset, and bevy_app.
/// It currently implements its own renderer which supports basic mesh drawing
///
/// To create a game with Fathom, you simply need to call [`FathomApplication::new`] which will
/// return a [`bevy_app::App`] that you can further configure as necessary just like other Bevy
/// apps
use bevy::app::{App, PluginGroupBuilder};
use bevy::ecs::schedule::ExecutorKind;
use bevy::prelude::{AppExit, Local, Mut, Plugin, PluginGroup, Schedule, World};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::{schedule, WinitApplicationState};
use crate::renderer::Fathom3DRenderPlugin;

pub mod app;
pub mod renderer;
pub mod assets;

struct FathomDefaultPlugins;

impl PluginGroup for FathomDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FathomRunnerPlugin)
            .add(Fathom3DRenderPlugin)
    }
}

struct FathomRunnerPlugin;

impl Plugin for FathomRunnerPlugin {
    fn build(&self, app: &mut App) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        app.insert_non_send_resource(event_loop);
        app.set_runner(fathom_app_runner);

        let mut main_schedule = Schedule::new(schedule::Main);
        // TODO: Figure out why bevy does this for "facilitator" schedules
        main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

        app.add_schedule(main_schedule)
            .init_resource::<schedule::MainScheduleOrder>()
            .add_systems(schedule::Main, run_main);

    }
}

fn fathom_app_runner(mut app: App) -> AppExit {
    let event_loop = app
        .world_mut()
        .remove_non_send_resource::<EventLoop<()>>()
        .unwrap();

    if let Err(error) = event_loop.run_app(&mut WinitApplicationState::new(app)) {
        log::error!("Error returned from the event loop's run_app method: {:?}", error);
        return AppExit::error();
    }

    AppExit::Success
}

pub fn run_main(world: &mut World, mut run_at_least_once: Local<bool>) {
    if !*run_at_least_once {
        world.resource_scope(|world, order: Mut<schedule::MainScheduleOrder>| {
            for &label in &order.startup_only_labels {
                let _ = world.try_run_schedule(label);
            }
        });
        *run_at_least_once = true;
    }

    world.resource_scope(|world, order: Mut<schedule::MainScheduleOrder>| {
        for &label in &order.non_startup_labels {
            let _ = world.try_run_schedule(label);
        }
    });
}