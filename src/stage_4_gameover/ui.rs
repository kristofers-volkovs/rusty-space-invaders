use bevy::prelude::*;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, ConditionSet, IntoConditionalSystem},
    state::NextState,
};

use crate::shared::{
    components::{ExitGameButton, GameRunning, ResetGameplay},
    constants::NORMAL_BUTTON,
    general::{button_color_system, despawn_system, on_button_interact},
    resources::{AppState, UiTextures},
};

use super::components::{GameOverMenu, RespawnButton};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::GameOver, setup_gameover_system)
            // --- State change ---
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::GameOver)
                    .with_system(respawn_system.run_if(on_button_interact::<RespawnButton>))
                    .into(),
            )
            // --- Basic button color changer ---
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::GameOver)
                    .with_system(button_color_system)
                    .into(),
            )
            // --- Ui cleanup ---
            .add_exit_system(AppState::GameOver, despawn_system::<GameOverMenu>);
    }
}

fn setup_gameover_system(mut commands: Commands, ui_textures: Res<UiTextures>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0.5, 0.5, 0.5, 0.2).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(180.), Val::Px(200.)),
                        border: Rect::all(Val::Px(2.)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.6, 0.6, 0.6).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                                        margin: Rect::all(Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: NORMAL_BUTTON.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Exit",
                                            TextStyle {
                                                font: ui_textures.ui_font.clone(),
                                                font_size: 40.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..default()
                                    });
                                })
                                .insert(ExitGameButton);

                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                                        margin: Rect::all(Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: NORMAL_BUTTON.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Respawn",
                                            TextStyle {
                                                font: ui_textures.ui_font.clone(),
                                                font_size: 40.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..default()
                                    });
                                })
                                .insert(RespawnButton);

                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Game over",
                                            TextStyle {
                                                font: ui_textures.ui_font.clone(),
                                                font_size: 60.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..default()
                                    });
                                });
                        });
                });
        })
        .insert(GameOverMenu);
}

fn respawn_system(mut commands: Commands) {
    commands.insert_resource(ResetGameplay);
    commands.insert_resource(NextState(AppState::Gameplay));
}
