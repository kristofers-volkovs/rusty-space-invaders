use bevy::prelude::*;

// Resources

pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
}

pub struct PlayerState {
    pub max_health: usize,
    pub health: usize,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            health: 3,
            max_health: 3,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self) {
        self.health -= 1;
    }
}
