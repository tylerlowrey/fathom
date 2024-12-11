use bevy::asset::{AssetEvents, AssetLoadFailedEvent, AssetServerMode, TrackAssets};
use bevy::asset::io::{AssetSourceBuilders};
use bevy::ecs::event::EventRegistry;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, ComputeTaskPool, IoTaskPool, TaskPoolBuilder};
use crate::app::schedule;
use crate::assets::materials::Material;
use crate::assets::shaders::{Shader, ShaderAssetLoader};

const DEFAULT_ASSETS_PATH: &str = "assets";
const DEFAULT_IO_THREADS_COUNT: usize = 2;
const DEFAULT_ASYNC_COMPUTE_THREADS_COUNT: usize = 2;
const DEFAULT_COMPUTE_THREADS_COUNT: usize = 2;

pub mod shaders;
pub mod materials;


pub fn initialize_asset_server(world: &mut World) {
    create_task_pools();

    let mut builders: AssetSourceBuilders = Default::default();
    builders.init_default_source(DEFAULT_ASSETS_PATH, None);
    let asset_sources = builders.build_sources(false, false);
    let asset_server = AssetServer::new(
        asset_sources,
        AssetServerMode::Unprocessed,
        false
    );

    let shader_assets = Assets::<Shader>::default();
    let shader_asset_loader = ShaderAssetLoader::from_world(world);
    asset_server.register_asset(&shader_assets);
    asset_server.register_loader(shader_asset_loader);

    asset_server.register_asset(&Assets::<Material>::default());

    world.insert_resource(asset_server);
    world.insert_resource(shader_assets);

    EventRegistry::register_event::<AssetEvent<Shader>>(world);
    EventRegistry::register_event::<AssetLoadFailedEvent<Shader>>(world);

    EventRegistry::register_event::<AssetEvent<Material>>(world);

    let registry = world.resource_mut::<AppTypeRegistry>();
    registry.write().register::<Handle<Shader>>();
    registry.write().register::<Handle<Material>>();

    let mut schedules = world.resource_mut::<Schedules>();
    schedules.add_systems(
        schedule::Last,
        Assets::<Shader>::asset_events
            .run_if(asset_events_condition::<Shader>)
            .in_set(AssetEvents)
    );
    schedules.add_systems(
        schedule::Last,
        Assets::<Shader>::track_assets.in_set(TrackAssets)
    );

    tick_task_pools();
}

pub fn create_task_pools() {
    let io_threads = DEFAULT_IO_THREADS_COUNT;
    let async_compute_threads = DEFAULT_ASYNC_COMPUTE_THREADS_COUNT;
    let compute_threads = DEFAULT_COMPUTE_THREADS_COUNT;

    log::debug!("number of IO Threads: {}", io_threads);
    log::debug!("number of async compute Threads: {}", io_threads);
    log::debug!("number of compute Threads: {}", io_threads);

    IoTaskPool::get_or_init(|| {
        TaskPoolBuilder::default()
            .num_threads(io_threads)
            .thread_name("IO Task Pool".to_string())
            .build()
    });

    AsyncComputeTaskPool::get_or_init(|| {
        TaskPoolBuilder::default()
            .num_threads(async_compute_threads)
            .thread_name("Async Compute Task Pool".to_string())
            .build()
    });

    ComputeTaskPool::get_or_init(|| {
        TaskPoolBuilder::default()
            .num_threads(compute_threads)
            .thread_name("Compute Task Pool".to_string())
            .build()
    });
}

pub fn tick_task_pools() {
    ComputeTaskPool::get()
        .with_local_executor(|compute_local_executor| {
            AsyncComputeTaskPool::get()
                .with_local_executor(|async_local_executor| {
                    IoTaskPool::get()
                        .with_local_executor(|io_local_executor| {
                            for _ in 0..100 {
                                compute_local_executor.try_tick();
                                async_local_executor.try_tick();
                                io_local_executor.try_tick();
                            }
                        });
                });
        });
}

// TODO: Use the Assets::<A>::asset_events_condition from bevy_asset instead (Can't do this currently because it is pub(crate) and not pub
pub fn asset_events_condition<A: Asset>(_assets: Res<Assets<A>>) -> bool {
    true
}