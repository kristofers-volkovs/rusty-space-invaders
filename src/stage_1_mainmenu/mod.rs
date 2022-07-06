use bevy::prelude::*;

use ui::MainMenuPlugin;

mod components;
mod ui;

pub struct MainMenuStage;

impl Plugin for MainMenuStage {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuPlugin);
    }
}
