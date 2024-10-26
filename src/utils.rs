use bevy::prelude::*;

// pub fn compute_cumulative_distances(path: &Vec<Vec2>) -> Vec<f32> {
//     let mut cumulative_distances = Vec::with_capacity(path.len());
//     let mut total_distance = 0.0;
//     cumulative_distances.push(0.0); // Starting point
//
//     for i in 1..path.len() {
//         let segment_length = (path[i] - path[i - 1]).length();
//         total_distance += segment_length;
//         cumulative_distances.push(total_distance);
//     }
//
//     cumulative_distances
// }
