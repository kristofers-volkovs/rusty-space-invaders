#![allow(unused)] // temp

use bevy::prelude::*;

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
        // --- Initial state ---
        // --- Stages ---
        .add_plugin(MainMenuStage)
        .add_plugin(GameplayStage)
        .add_plugin(PausedStage)
        .add_plugin(GameOverStage)
        .run();
}
