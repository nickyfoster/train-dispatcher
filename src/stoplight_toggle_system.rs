use bevy::prelude::*;

use crate::components::*;

pub fn stoplight_toggle_system(
    mut stoplight_query: Query<(&mut Stoplight, &mut Sprite)>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StoplightButton>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Toggle the stoplight state
            for (mut stoplight, mut sprite) in stoplight_query.iter_mut() {
                stoplight.is_red = !stoplight.is_red;

                // Update the stoplight sprite color
                sprite.color = if stoplight.is_red {
                    Color::rgb(1.0, 0.0, 0.0) // Red
                } else {
                    Color::rgb(0.0, 1.0, 0.0) // Green
                };

                println!(
                    "Stoplight is now {}",
                    if stoplight.is_red { "Red" } else { "Green" }
                );
            }

            // Update the button's background color
            *background_color =
                BackgroundColor(if stoplight_query.iter().next().unwrap().0.is_red {
                    Color::rgb(0.8, 0.2, 0.2) // Red
                } else {
                    Color::rgb(0.2, 0.8, 0.2) // Green
                });
        }
    }
}
