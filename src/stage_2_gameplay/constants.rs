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

// Game Constants

pub const TIME_STEP: f32 = 1. / 60.;
pub const BASE_SPEED: f32 = 500.;

pub const PLAYER_RESPAWN_DELAY: f64 = 2.;
pub const ENEMY_MAX: u32 = 2;
pub const FORMATION_MEMBERS_MAX: u32 = 2;

// Labels

pub const PLAYER_SPAWN: &str = "player_spawn";
pub const ENEMY_SPAWN: &str = "enemy_spawn";
