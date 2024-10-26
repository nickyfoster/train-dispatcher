use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use serde_json::from_str;
use std::fs::read_to_string;

use crate::components::*;

fn compute_cumulative_distances(path: &Vec<Vec2>) -> Vec<f32> {
    let mut cumulative_distances = Vec::with_capacity(path.len());
    let mut total_distance = 0.0;
    cumulative_distances.push(0.0); // Starting point

    for i in 1..path.len() {
        let segment_length = (path[i] - path[i - 1]).length();
        total_distance += segment_length;
        cumulative_distances.push(total_distance);
    }

    cumulative_distances
}

pub fn setup(mut commands: Commands) {
    // Load track configuration
    let config_data = read_to_string("config.json").expect("Unable to read config file");
    let track_config: TrackConfig = from_str(&config_data).expect("Invalid configuration file");

    // Convert track points to Vec<Vec2>
    let track_points: Vec<Vec2> = track_config
        .track_points
        .iter()
        .map(|&[x, y]| Vec2::new(x, y))
        .collect();

    // Create the track path
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(track_points[0]);
    for point in &track_points[1..] {
        path_builder.line_to(*point);
    }
    let track_path = path_builder.build();
    let cumulative_distances = compute_cumulative_distances(&track_points);

    // Calculate track bounds
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for point in &track_points {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    let track_center = Vec2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);

    // Camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(track_center.extend(999.9)),
        ..default()
    });

    // Track
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&track_path),
            ..default()
        },
        Stroke::new(Color::rgb(128.0, 128.0, 128.0), 5.0),
        Track {
            path: track_points.clone(),
        },
    ));

    // Train
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(30.0, 15.0)),
                ..default()
            },
            transform: Transform {
                translation: track_points[0].extend(1.0),
                ..default()
            },
            ..default()
        },
        Train {
            speed: 100.0,
            current_speed: 0.0,
            acceleration: 50.0,
            deceleration: 70.0,
            stop_distance: 10.0,
            path_index: 0,
            path_progress: 0.0,
            path: track_points.clone(),
            cumulative_distances,
        },
    ));

    let stoplight_position = Vec2::new(
        track_config.stoplight_position[0],
        track_config.stoplight_position[1],
    );

    // Stoplight Entity
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0), // Initially red
                custom_size: Some(Vec2::new(20.0, 50.0)),
                ..default()
            },
            transform: Transform {
                translation: stoplight_position.extend(1.0),
                ..default()
            },
            ..default()
        },
        Stoplight {
            is_red: true,
            position: stoplight_position,
        },
        Name::new("Stoplight"),
    ));

    // Controls Panel
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            // Stoplight Toggle Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.8, 0.2, 0.2)), // Red color
                        ..default()
                    },
                    StoplightButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "S1",
                            TextStyle {
                                font: Default::default(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                });
        });
}
