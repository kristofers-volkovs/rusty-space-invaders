use bevy::prelude::*;

use super::constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

// Basic button color match
pub fn button_color_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// Despawns all entities that have a specific component attached to it
pub fn despawn_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// System for handling multiple buttons
//
// Returns true if component with the given component is clicked
pub fn on_button_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

// Key presses
pub fn esc_pressed(kdb: Res<Input<KeyCode>>) -> bool {
    kdb.just_pressed(KeyCode::Escape)
}
