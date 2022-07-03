use bevy::prelude::*;

use enemy::EnemyPlugin;
use general::GeneralPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

pub mod components;
pub mod enemy;
pub mod general;
pub mod player;
pub mod ui;

pub struct GameplayStage;

impl Plugin for GameplayStage {
    fn build(&self, app: &mut App) {
        app.add_plugin(GeneralPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(UiPlugin);
    }
}
