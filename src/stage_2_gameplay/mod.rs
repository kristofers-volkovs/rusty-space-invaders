use bevy::prelude::*;

use enemy::EnemyPlugin;
use general::GeneralPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

mod components;
mod constants;
mod enemy;
mod general;
mod player;
mod resources;
mod ui;

pub struct GameplayStage;

impl Plugin for GameplayStage {
    fn build(&self, app: &mut App) {
        app.add_plugin(GeneralPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(UiPlugin);
    }
}
