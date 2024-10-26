use crate::components::TrackConfig;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use serde_json::from_str;
use std::fs::read_to_string;

pub fn setup(mut commands: Commands) {
    let config_data = read_to_string("config.json").expect("Unable to read config file");
    let track_config: TrackConfig = from_str(&config_data).expect("Invalid configuration file");

    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 999.9),
            ..default()
        },
        ..default()
    });

    for track in &track_config.tracks {
        let mut track_path_builder = PathBuilder::new();

        for path in &track.paths {
            if let Some(start_point) = path.coordinates.first() {
                track_path_builder.move_to(Vec2::new(start_point[0], start_point[1]));

                for &coordinate in &path.coordinates[1..] {
                    track_path_builder.line_to(Vec2::new(coordinate[0], coordinate[1]));
                }
            }
        }

        let track_path = track_path_builder.build();
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&track_path),
                ..default()
            },
            Stroke::new(Color::rgb(128.0, 128.0, 128.0), 5.0),
            TrackComponent { id: track.id },
        ));
    }
}

#[derive(Component)]
struct TrackComponent {
    id: u32,
}
