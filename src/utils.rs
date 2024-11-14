use crate::components::Config;
use bevy::prelude::*;
use serde_json::from_str;
use std::fs::read_to_string;

pub fn load_config(file_path: &str) -> Config {
    let config_data = read_to_string(file_path).expect("Unable to read config file");
    from_str(&config_data).expect("Invalid configuration file")
}

pub fn get_track_center(config: &Config) -> Vec2 {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for track in &config.tracks {
        for wp in &track.coordinates {
            if wp.x < min_x {
                min_x = wp.x;
            }
            if wp.x > max_x {
                max_x = wp.x;
            }
            if wp.y < min_y {
                min_y = wp.y;
            }
            if wp.y > max_y {
                max_y = wp.y;
            }
        }
    }
    Vec2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0)
}
