use bevy::prelude::*;

use crate::components::*;

pub fn train_movement_system(
    time: Res<Time>,
    mut train_query: Query<(&mut Transform, &mut Train)>,
    stoplight_query: Query<&Stoplight>,
) {
    let stoplight = match stoplight_query.get_single() {
        Ok(stoplight) => stoplight,
        Err(_) => return, // If no stoplight is found, skip this system
    };

    for (mut transform, mut train) in train_query.iter_mut() {
        if train.path.len() < 2 {
            continue; // Not enough points to form a path
        }

        // Calculate current position on the path
        let current_point = train.path[train.path_index];
        let next_point = train.path[(train.path_index + 1) % train.path.len()];
        let segment_vector = next_point - current_point;
        let segment_length = segment_vector.length();

        // Position along the segment
        let t = train.path_progress / segment_length;
        let position = current_point + segment_vector * t;

        // Distance to the stoplight
        let distance_to_stoplight = position.distance(stoplight.position);

        // Calculate required stopping distance based on current speed and deceleration
        let required_stopping_distance = if train.deceleration > 0.0 {
            (train.current_speed * train.current_speed) / (2.0 * train.deceleration)
        } else {
            0.0
        };

        // Determine if train should start decelerating
        let should_slow_down =
            stoplight.is_red && distance_to_stoplight <= required_stopping_distance;

        // Update train's speed based on whether it should stop or start moving
        if should_slow_down {
            // Decelerate the train smoothly
            train.current_speed -= train.deceleration * time.delta_seconds();
            if train.current_speed < 0.0 {
                train.current_speed = 0.0; // Ensure speed doesn't go negative
            }
        } else if stoplight.is_red && distance_to_stoplight < train.stop_distance {
            train.current_speed = 0.0;
        } else {
            // Accelerate the train smoothly up to its maximum speed
            train.current_speed += train.acceleration * time.delta_seconds();
            if train.current_speed > train.speed {
                train.current_speed = train.speed; // Cap the speed at maximum
            }
        }

        // Move the train along the path if it's moving
        if train.current_speed > 0.0 {
            let mut distance_to_move = train.current_speed * time.delta_seconds();

            while distance_to_move > 0.0 {
                let current_point = train.path[train.path_index];
                let next_index = (train.path_index + 1) % train.path.len();
                let next_point = train.path[next_index];

                let segment_vector = next_point - current_point;
                let segment_length = segment_vector.length();

                if segment_length == 0.0 {
                    // Skip zero-length segments
                    train.path_index = next_index;
                    train.path_progress = 0.0;
                    continue;
                }

                let remaining_distance_on_segment = segment_length - train.path_progress;

                if distance_to_move < remaining_distance_on_segment {
                    train.path_progress += distance_to_move;
                    distance_to_move = 0.0;
                } else {
                    distance_to_move -= remaining_distance_on_segment;
                    train.path_index = next_index;
                    train.path_progress = 0.0;
                }
            }

            // Update position and rotation after movement
            let current_point = train.path[train.path_index];
            let next_point = train.path[(train.path_index + 1) % train.path.len()];
            let segment_vector = next_point - current_point;
            let segment_length = segment_vector.length();

            if segment_length != 0.0 {
                let t = train.path_progress / segment_length;
                let new_position = current_point + segment_vector * t;

                transform.translation = new_position.extend(1.0);

                // Update rotation to face movement direction
                let direction = segment_vector.normalize();
                let angle = direction.y.atan2(direction.x);
                transform.rotation = Quat::from_rotation_z(angle);
            }
        } else {
            // Ensure train's position is updated even when stopped
            transform.translation = position.extend(1.0);
        }

        // Update rotation to face movement direction
        if segment_vector.length_squared() != 0.0 {
            let direction = segment_vector.normalize();
            let angle = direction.y.atan2(direction.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
