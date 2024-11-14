#![allow(unused_mut, unused_variables, unused_imports, dead_code, deprecated)]
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_prototype_lyon::prelude::*;

mod components;
mod setup;
mod train_movement_system;
mod utils;

use setup::setup;
use train_movement_system::train_movement_system;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TRAIN DISPATCHER".into(),
                        resolution: (1280., 720.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                }),
        )
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, train_movement_system)
        .run();
}
