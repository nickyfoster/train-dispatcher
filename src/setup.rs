use crate::components::{Config, Track, Train, TrainDirectionButton};
use crate::utils::{get_track_center, load_config};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform::commands,
};
use bevy_prototype_lyon::prelude::*;

pub fn setup(mut commands: Commands) {
    let config = load_config("config.json");

    spawn_tracks(&config, &mut commands);
    spawn_switches(&config, &mut commands);
    spawn_camera(&config, &mut commands);
    spawn_train(&config, &mut commands);
    spawn_control_panel(&config, &mut commands);
}

fn spawn_tracks(config: &Config, commands: &mut Commands) {
    for track in &config.tracks {
        let mut track_path_builder = PathBuilder::new();
        let mut first_point_set = false;
        for vec in &track.coordinates {
            if !first_point_set {
                track_path_builder.move_to(vec.clone());
                first_point_set = true;
            } else {
                track_path_builder.line_to(vec.clone());
            }
        }
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&track_path_builder.build()),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                    ..default()
                },
                ..default()
            },
            Stroke::new(Color::rgb(100.0, 100.0, 0.0), 2.0),
        ));
    }
}

fn spawn_switches(config: &Config, commands: &mut Commands) {
    for switch in &config.switches {
        let mut switch_path_builder = PathBuilder::new();
        for exit in &switch.exits {
            switch_path_builder.move_to(Vec2::new(switch.enter.x, switch.enter.y));
            switch_path_builder.line_to(Vec2::new(exit.x, exit.y));
        }
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&switch_path_builder.build()),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                },
                ..default()
            },
            Stroke::new(Color::rgb(100.0, 0.0, 100.0), 3.0),
        ));
    }
}

fn spawn_camera(config: &Config, commands: &mut Commands) {
    let track_center = get_track_center(config);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(track_center.extend(999.9)),
        ..default()
    });
}

fn spawn_train(config: &Config, commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(100.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 3.0),
                ..default()
            },
            ..default()
        },
        Train {
            id: 0,
            location: config.tracks[0].coordinates[0].clone(),
            config: config.clone(),
            ..default()
        },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(100.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 3.0),
                ..default()
            },
            ..default()
        },
        Train {
            id: 1,
            location: config.tracks[0].coordinates[0].clone(),
            config: config.clone(),
            track_id: 1,
            speed: 160.0,
            direction: "backward".to_string(),
            ..default()
        },
    ));

    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: Mesh2dHandle(Circle { radius: 50.0 }),
    //     material: materials.add(color),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
}

fn spawn_control_panel(config: &Config, commands: &mut Commands) {
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
                    TrainDirectionButton,
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
