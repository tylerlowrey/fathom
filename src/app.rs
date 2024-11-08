use std::sync::Arc;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::error::EventLoopError;
use log::info;
use crate::app::schedule::{Startup, Render, Update, PreRender, Initialization};
use crate::assets::{initialize_asset_server};
use crate::renderer::{add_default_2d_render_resources, add_default_render_resources, initialize_render_resources, initialize_renderer, pre_render, render, render2d};
use crate::renderer::mesh::{setup_on_add_hook_for_mesh, setup_on_add_hook_for_mesh2d};

pub struct GameApplication {
    world: World,
    startup_finished: bool,
}

#[derive(Resource)]
pub struct WindowState {
    window: Arc<winit::window::Window>,
}

impl GameApplication {
    pub fn new() -> Self {
        let mut world = World::new();
        world.add_schedule(Schedule::new(Initialization));
        world.add_schedule(Schedule::new(Startup));
        world.add_schedule(Schedule::new(PostStartup));
        world.add_schedule(Schedule::new(Update));
        world.add_schedule(Schedule::new(PreRender));
        world.add_schedule(Schedule::new(Render));

        Self {
            world,
            startup_finished: false,
        }
    }

    /// Creates a winit event loop and runs it, passing this app to it
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)
    }

    pub fn add_system_to_schedule<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        system: impl IntoSystemConfigs<M>
    ) {
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.add_systems(schedule, system);
    }

    pub fn add_renderer_2d(&mut self) {
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.add_systems(Initialization, (
            initialize_renderer,
            initialize_asset_server,
            initialize_render_resources,
            add_default_2d_render_resources,
            setup_on_add_hook_for_mesh2d
        ).chain());
        schedules.add_systems(PreRender, pre_render);
        schedules.add_systems(Render, render2d);
    }

    pub fn add_renderer_3d(&mut self) {
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.add_systems(Initialization, (
            initialize_renderer,
            initialize_asset_server,
            initialize_render_resources,
            add_default_render_resources,
            setup_on_add_hook_for_mesh
        ).chain());
        schedules.add_systems(PreRender, pre_render);
        schedules.add_systems(Render, render);
    }
}

impl ApplicationHandler for GameApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Resumed event called");
        let window_attributes = winit::window::Window::default_attributes()
            .with_title("Fathom Game Engine");
        let window = event_loop.create_window(window_attributes)
            .expect("Failed to create winit window");
        self.world.insert_resource(WindowState::new(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Close requested. Closing application...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let world = &mut self.world;
                if !self.startup_finished {
                    world.run_schedule(Initialization);
                    world.run_schedule(Startup);
                    world.run_schedule(PostStartup);
                    self.startup_finished = true;
                }
                world.run_schedule(Update);
                world.run_schedule(PreRender);
                world.run_schedule(Render);
            }
            _ => ()
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.world.resource::<WindowState>().window();
        window.request_redraw();
    }
}

impl WindowState {
    pub fn new(window: winit::window::Window) -> Self {
        let window = Arc::new(window);

        Self {
            window: window.clone(),
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn clone_window(&self) -> Arc<winit::window::Window> {
        self.window.clone()
    }
}

pub mod schedule {
    use bevy::ecs::schedule::ScheduleLabel;

    /// This schedule gets run first before everything else and initializes the engine. Game systems
    /// usually should not run during this schedule
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Initialization;

    /// This schedule gets run once before Update, PreRender, and Render get run for the first time
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Startup;

    /// This schedule gets run once after Startup
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct PostStartup;

    /// This schedule gets run before the render schedule. Systems that do not impact rendering should go here
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Update;

    /// This schedule gets run after Update and before Render
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct PreRender;

    /// This schedule gets run last on each event loop cycle. Systems that render MUST go here
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Render;
}
