use bevy::{
    core::Timer,
    math::{Vec2, Vec3},
    prelude::Component,
};

// Common Components

#[derive(Clone, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct Invincibility(pub f32);

impl From<f32> for Invincibility {
    fn from(val: f32) -> Self {
        Invincibility(val)
    }
}

#[derive(Component)]
pub struct InvincibilityTimer(pub Timer);

impl Default for InvincibilityTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, true))
    }
}

#[derive(Component)]
pub struct FiringCooldownTimer(pub Timer);

impl Default for FiringCooldownTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, false))
    }
}

// Player Components

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

// Explosion Components

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, true))
    }
}

// Ui Components

#[derive(Component)]
pub struct HeartImage;
