use bevy::prelude::*;

use ui::GameOverPlugin;

mod components;
mod ui;

pub struct GameOverStage;

impl Plugin for GameOverStage {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameOverPlugin);
    }
}
