use std::sync::Arc;
use bevy::ecs::event::{event_update_condition, event_update_system, EventUpdates};
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::error::EventLoopError;
use log::info;
use crate::app::schedule::{Startup, Render, Update, PreRender, Initialization};
use crate::assets::{initialize_asset_server, tick_task_pools};
use crate::FathomDefaultPlugins;
use crate::renderer::{add_default_2d_render_resources, add_default_render_resources, initialize_render_resources, initialize_renderer, pre_render, render3d, render2d, Fathom3DRenderPlugin, Fathom2DRenderPlugin};
use crate::renderer::mesh::{setup_on_add_hook_for_mesh, setup_on_add_hook_for_mesh2d};

pub struct FathomApplication;

impl FathomApplication {
    pub fn new() -> App {
        let mut app = App::empty();
        app.main_mut().update_schedule = Some(schedule::Main.intern());
        app.init_resource::<AppTypeRegistry>();
        app.add_systems(
            First,
            event_update_system
                .in_set(EventUpdates)
                .run_if(event_update_condition),
        );
        app.add_event::<AppExit>();

        // Disable the Fathom3DRenderPlugin because this function should return a barebones
        // fathom application but don't want to modify FathomDefaultPlugins itself
        app.add_plugins(
            FathomDefaultPlugins.build()
                .disable::<Fathom3DRenderPlugin>()
        );

        app
    }

    pub fn with_3d_renderer() -> App {
        let mut app = Self::new();
        app.add_plugins(Fathom3DRenderPlugin);
        app
    }

    pub fn with_2d_renderer() -> App {
        let mut app = Self::new();
        app.add_plugins(Fathom2DRenderPlugin);
        app
    }
}

pub struct WinitApplicationState {
    app: App,
}

#[derive(Resource)]
pub struct WindowState {
    window: Arc<winit::window::Window>,
}

impl WinitApplicationState {
    pub fn new(app: App) -> Self {
        Self {
            app
        }
    }

    pub fn add_renderer_2d(&mut self) {
        let mut schedules = self.app.world_mut().resource_mut::<Schedules>();
        schedules.add_systems(Initialization, (
            initialize_renderer,
            initialize_asset_server,
            initialize_render_resources,
            add_default_2d_render_resources,
            setup_on_add_hook_for_mesh2d
        ).chain());
        schedules.add_systems(PreRender, pre_render);
        schedules.add_systems(Render, render2d);
        schedules.add_systems(Last, tick_task_pools);
    }

    pub fn add_renderer_3d(&mut self) {
        let mut schedules = self.app.world_mut().resource_mut::<Schedules>();
        schedules.add_systems(Initialization, (
            initialize_renderer,
            initialize_asset_server,
            initialize_render_resources,
            add_default_render_resources,
            setup_on_add_hook_for_mesh
        ).chain());
        schedules.add_systems(PreRender, pre_render);
        schedules.add_systems(Render, render3d);
        schedules.add_systems(Last, tick_task_pools);
    }
}

impl ApplicationHandler for WinitApplicationState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Resumed event called");
        let window_attributes = winit::window::Window::default_attributes()
            .with_title("Fathom Game Engine");
        let window = event_loop.create_window(window_attributes)
            .expect("Failed to create winit window");
        self.app.world_mut().insert_resource(WindowState::new(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Close requested. Closing application...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.app.update();
            }
            _ => ()
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.app.world().resource::<WindowState>().window();
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
    use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
    use bevy::prelude::Resource;

    /// This is the schedule that contains all the other schedules, it is set as the main app's update_schedule
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Main;

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

    /// This schedule gets run first before all the other repeatedly run schedules (Update, etc.)
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct First;

    /// This schedule gets run before the render schedule. Systems that do not impact rendering should go here
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Update;

    /// This schedule gets run after Update and before Render
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct PreRender;

    /// This schedule gets run right before the last on each event loop cycle. Systems that render MUST go here
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Render;

    /// This schedule is always run last
    #[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Last;

    #[derive(Resource, Debug)]
    pub struct MainScheduleOrder {
        /// ScheduleLabels that should be run repeatedly after startup
        pub non_startup_labels: Vec<InternedScheduleLabel>,
        /// ScheduleLabels that should on be run once at startup
        pub startup_only_labels: Vec<InternedScheduleLabel>,
    }

    impl Default for MainScheduleOrder {
        fn default() -> Self {
            Self {
                non_startup_labels: vec![
                    First.intern(),
                    Update.intern(),
                    PreRender.intern(),
                    Render.intern(),
                    Last.intern(),
                ],
                startup_only_labels: vec![
                    Initialization.intern(),
                    Startup.intern(),
                    PostStartup.intern()
                ],
            }
        }
    }


}
