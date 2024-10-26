use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::*;

mod components;
mod setup;
mod utils;

use setup::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TRAIN DISPATCHER".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .run();
}
