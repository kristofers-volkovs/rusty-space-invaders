use bevy::prelude::*;

use ui::PauseMenuPlugin;

mod components;
mod ui;

pub struct PausedStage;

impl Plugin for PausedStage {
    fn build(&self, app: &mut App) {
        app.add_plugin(PauseMenuPlugin);
    }
}
