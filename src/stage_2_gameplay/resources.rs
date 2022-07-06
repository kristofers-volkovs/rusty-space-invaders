use bevy::prelude::*;

// Resources

pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
}

pub struct EnemyCount(pub u32);

pub struct PlayerState {
    pub on: bool,       // alive
    pub last_shot: f64, // last time died, -1 if not shot
    pub max_health: usize,
    pub health: usize,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
            health: 3,
            max_health: 3,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
        self.health -= 1;
    }

    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
    }
}
