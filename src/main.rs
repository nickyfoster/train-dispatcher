use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::*;

mod components;
mod setup;
mod stoplight_toggle_system;
mod train_movement_system;

use setup::setup;
use stoplight_toggle_system::stoplight_toggle_system;
use train_movement_system::train_movement_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, train_movement_system)
        .add_systems(Update, stoplight_toggle_system)
        .run();
}
