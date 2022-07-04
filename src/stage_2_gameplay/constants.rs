use bevy::prelude::*;

// Asset Constants

pub const PLAYER_SPRITE: &str = "player_a_01.png";
pub const PLAYER_SIZE: (f32, f32) = (144., 75.);
pub const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
pub const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

pub const ENEMY_SPRITE: &str = "enemy_a_01.png";
pub const ENEMY_SIZE: (f32, f32) = (144., 75.);
pub const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
pub const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

pub const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
pub const EXPLOSION_LEN: usize = 16;

pub const SPRITE_SCALE: f32 = 0.5;

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

// Game Constants

pub const TIME_STEP: f32 = 1. / 60.;
pub const BASE_SPEED: f32 = 500.;

pub const PLAYER_RESPAWN_DELAY: f64 = 2.;
pub const ENEMY_MAX: u32 = 2;
pub const FORMATION_MEMBERS_MAX: u32 = 2;
