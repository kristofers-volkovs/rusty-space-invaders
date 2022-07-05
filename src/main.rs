#![allow(unused)] // temp

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use common::constants::{UiTextures, WinSize, GAME_FONT, PLAYER_HEART_EMPTY, PLAYER_HEART_FULL};
use common::AppState;

use stage_1_mainmenu::MainMenuStage;
use stage_2_gameplay::GameplayStage;
use stage_3_paused::PausedStage;
use stage_4_gameover::GameOverStage;

mod common;
mod stage_1_mainmenu;
mod stage_2_gameplay;
mod stage_3_paused;
mod stage_4_gameover;

fn main() {
    App::new()
        // --- Game initial config ---
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rusty Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // --- General systems ---
        .add_startup_system(setup_system)
        // --- Initial state ---
        .add_loopless_state(AppState::Gameplay)
        // --- Stages ---
        .add_plugin(MainMenuStage)
        .add_plugin(GameplayStage)
        .add_plugin(PausedStage)
        .add_plugin(GameOverStage)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // add UiTextures resource
    let ui_textures = UiTextures {
        heart_full: asset_server.load(PLAYER_HEART_FULL),
        heart_empty: asset_server.load(PLAYER_HEART_EMPTY),
        ui_font: asset_server.load(GAME_FONT),
    };

    commands.insert_resource(ui_textures);
}
