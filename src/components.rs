use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct Stoplight {
    pub is_red: bool,
    pub position: Vec2,
}

#[derive(Component)]
pub struct StoplightButton;

#[derive(Deserialize)]
pub struct TrackConfig {
    pub track_points: Vec<[f32; 2]>,
    pub stoplight_position: [f32; 2],
}

#[derive(Component)]
pub struct Track {
    pub path: Vec<Vec2>,
}

#[derive(Component)]
pub struct Train {
    pub speed: f32,
    pub current_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub stop_distance: f32,
    pub path_index: usize,
    pub path_progress: f32,
    pub path: Vec<Vec2>,
    pub cumulative_distances: Vec<f32>,
}
