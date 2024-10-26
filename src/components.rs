use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct StoplightButton;

#[derive(Deserialize, Debug)]
pub struct TrackConfig {
    pub tracks: Vec<Track>,
    pub stoplights: Vec<Stoplight>,
    pub switches: Vec<Switch>,
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

#[derive(Deserialize, Debug)]
pub struct Track {
    pub id: u32,
    pub paths: Vec<Path>,
}

#[derive(Deserialize, Debug)]
pub struct Path {
    pub id: u32,
    pub mode: String,
    pub coordinates: Vec<[f32; 2]>,
}

#[derive(Deserialize, Debug)]
pub struct Stoplight {
    pub id: u32,
    pub name: String,
    pub mode: String,
    pub coordinates: [f32; 2],
}

#[derive(Deserialize, Debug)]
pub struct Switch {
    pub id: u32,
    pub name: String,
    pub enter: [f32; 2],
    pub exit_primary: [f32; 2],
    pub exit_secondary: [f32; 2],
}
