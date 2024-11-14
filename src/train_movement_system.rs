use bevy::prelude::*;

use crate::components::*;

pub fn train_movement_system(
    time: Res<Time>,
    mut train_query: Query<(&mut Transform, &mut Train)>,
) {
    for (mut transform, mut train) in train_query.iter_mut() {
        let tracks = train.config.tracks.clone();
        let track = &tracks[train.track_id];
        let coordinates_len = track.coordinates.len();

        // TODO: implement [a|de]cceleration logic
        train.current_speed = train.speed;

        let mut distance_to_move = train.current_speed * time.delta_seconds();

        while distance_to_move > 0.0 {
            let current_index = train.track_index;
            let current_point = track.coordinates[train.track_index];

            let next_index = match train.direction.as_str() {
                "forward" => (current_index + 1) % coordinates_len,
                "backward" => {
                    if current_index == 0 {
                        coordinates_len - 1
                    } else {
                        current_index - 1
                    }
                }
                _ => current_index,
            };

            let next_point = track.coordinates[next_index];

            let segment_vector = if train.direction == "forward" {
                next_point - current_point
            } else {
                current_point - next_point
            };
            let segment_length = segment_vector.length();

            if segment_length == 0.0 {
                // Skip zero-length segments
                train.track_index = next_index;
                train.path_progress = 0.0;
                continue;
            }

            let remaining_distance_on_segment = segment_length - train.path_progress;
            if distance_to_move < remaining_distance_on_segment {
                train.path_progress += distance_to_move;
                distance_to_move = 0.0;
            } else {
                distance_to_move -= remaining_distance_on_segment;
                train.track_index = next_index;
                train.path_progress = 0.0;
            }

            // Update position
            let new_current_point = track.coordinates[train.track_index];
            let new_next_index = match train.direction.as_str() {
                "forward" => (train.track_index + 1) % coordinates_len,
                "backward" => {
                    if train.track_index == 0 {
                        coordinates_len - 1
                    } else {
                        train.track_index - 1
                    }
                }
                _ => train.track_index,
            };
            let new_next_point = track.coordinates[new_next_index];

            let new_segment_vector = new_next_point - new_current_point;
            let new_segment_length = new_segment_vector.length();

            if segment_length != 0.0 {
                let t = train.path_progress / new_segment_length;
                let new_position = new_current_point + new_segment_vector * t;

                transform.translation = new_position.extend(3.0);
            }
        }
    }
}
