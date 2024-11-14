#[warn(unused_imports)]
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub tracks: Vec<Track>,
    pub switches: Vec<Switch>,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[warn(unused_variables)]
pub struct Track {
    pub id: u32,
    pub coordinates: Vec<Vec2>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Switch {
    pub id: u32,
    pub enter: Vec2,
    pub exits: Vec<Vec2>,
}

impl Default for Switch {
    fn default() -> Self {
        Switch {
            id: 0,
            enter: Vec2::new(0.0, 0.0),
            exits: Vec::new(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Train {
    pub id: u32,
    pub location: Vec2,
    pub config: Config,
    pub track_index: usize,
    pub track_id: usize,
    pub path_progress: f32,
    pub speed: f32,
    pub current_speed: f32,
    pub direction: String,
}

impl Default for Train {
    fn default() -> Self {
        Train {
            id: 0,
            location: Vec2::new(0.0, 0.0),
            config: Config::default(),
            track_index: 0,
            track_id: 0,
            path_progress: 0.0,
            speed: 100.0,
            current_speed: 0.0,
            direction: "forward".to_string(),
        }
    }
}

#[derive(Component)]
pub struct TrainDirectionButton;
