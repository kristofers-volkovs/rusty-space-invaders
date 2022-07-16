use bevy::prelude::*;

use crate::stage_2_gameplay::components::Point;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;


#[derive(Debug, Clone, Component)]
pub struct Formation {
    pub radius: (f32, f32),
    pub pivot: Point,
    pub start: Point,
}

#[derive(Component, Clone, Debug)]
pub enum EnemyMovementState {
    Stationary,
    Downward,
    Travel(Point),
    Seeking,
    Circle(Formation),
}

impl Default for EnemyMovementState {
    fn default() -> Self {
        EnemyMovementState::Stationary
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct EnemyMovement {
    pub speed: f32,
    pub angle: f32,
    pub state: EnemyMovementState,
}

#[derive(Component, Clone, Debug, Default)]
pub struct EnemyStats {
    pub health: usize,
    pub spawn_rate: f32,  // from 0 to 1
    pub firing_rate: f32, // from 0 to 1
}

#[derive(Bundle, Default, Clone, Debug)]
pub struct EnemyBundle {
    pub stats: EnemyStats,
    pub movement: EnemyMovement,
}

pub enum SpawningDirection {
    Top,
    Sides,
}

pub struct EnemyCount {
    pub asteroids: u32,
    pub minions: u32,
}

impl Default for EnemyCount {
    fn default() -> Self {
        EnemyCount {
            asteroids: 0,
            minions: 0,
        }
    }
}
