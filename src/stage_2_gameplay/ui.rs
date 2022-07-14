use bevy::prelude::*;
use iyes_loopless::prelude::{
    AppLooplessStateExt, ConditionHelpers, ConditionSet, IntoConditionalSystem,
};
use iyes_loopless::state::NextState;

use crate::shared::components::{ExitGameButton, GameRunning, GameplayTeardown, ResetGameplay};
use crate::shared::general::{esc_pressed, on_button_interact};
use crate::shared::resources::{AppState, UiTextures, WinSize};
use crate::stage_2_gameplay::components::HeartImage;
use crate::stage_2_gameplay::resources::PlayerState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // --- Sets up the games ui ---
        app.add_system(
            setup_ui_system
                .run_in_state(AppState::Gameplay)
                .run_if_resource_added::<GameRunning>(),
        )
        // --- Ui and state main systems ---
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Gameplay)
                // updates players current health
                .with_system(heart_image_update_system)
                // esc pauses the game
                .with_system(pause_system.run_if(esc_pressed))
                // when player dies the game over screen pops up
                .with_system(
                    game_over_system
                        .run_if(has_player_died)
                        .run_unless_resource_exists::<ResetGameplay>(),
                )
                .into(),
        )
        // --- Gameplay teardown and exit to MainMenu ---
        .add_system_set(
            SystemSet::new()
                .with_system(
                    gameplay_to_clean_up_system.run_if(on_button_interact::<ExitGameButton>),
                )
                .with_system(
                    teardown_system::<GameplayTeardown>
                        .run_if_resource_exists::<GameplayTeardown>(),
                )
                .with_system(exit_gameplay_system.run_if_resource_removed::<GameplayTeardown>()),
        );
    }
}

fn setup_ui_system(
    mut commands: Commands,
    ui_textures: Res<UiTextures>,
    player_state: Res<PlayerState>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(130.), Val::Percent(100.)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::SpaceBetween,
                        border: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    for i in 1..=player_state.max_health {
                        let bundle = heart_image_bundle(ui_textures.heart_full.clone().into());
                        if i > player_state.health {
                            let bundle = heart_image_bundle(ui_textures.heart_empty.clone().into());
                        }
                        parent.spawn_bundle(bundle).insert(HeartImage);
                    }
                });
        });
}

fn heart_image_bundle(image: UiImage) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Px(40.), Val::Px(40.)),
            ..Default::default()
        },
        image,
        ..Default::default()
    }
}

fn heart_image_update_system(
    player_state: Res<PlayerState>,
    ui_textures: Res<UiTextures>,
    mut query: Query<&mut UiImage, With<HeartImage>>,
) {
    for (idx, mut image) in query.iter_mut().enumerate() {
        if idx >= player_state.health {
            image.0 = ui_textures.heart_empty.clone().into();
        } else {
            image.0 = ui_textures.heart_full.clone().into();
        }
    }
}

fn pause_system(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::Paused));
}

// Player death systems

fn game_over_system(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::GameOver));
}

fn has_player_died(mut commands: Commands, player_state: Res<PlayerState>) -> bool {
    player_state.health == 0
}

// Gameplay teardown and state change to MainMenu

fn gameplay_to_clean_up_system(mut commands: Commands) {
    commands.remove_resource::<GameRunning>();
    commands.insert_resource(GameplayTeardown);
}

fn teardown_system<T: Component>(mut commands: Commands, query: Query<Entity, Without<(Camera)>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<T>()
}

fn exit_gameplay_system(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::MainMenu));
}
