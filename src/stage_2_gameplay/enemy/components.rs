use bevy::prelude::*;

use crate::stage_2_gameplay::components::{EntityType, Point};

#[derive(Component)]
pub struct Enemy;

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

pub enum SpawningDirection {
    Top,
    Sides,
}

// Bundles

#[derive(Bundle, Default, Clone, Debug)]
pub struct EnemyBundle {
    pub stats: EnemyStats,
    pub movement: EnemyMovement,
    pub enemy_type: EntityType,
}

// Events

pub struct EnemySpawnEvent(pub SpawnEnemy);

pub struct SpawnEnemy {
    pub bundle: EnemyBundle,
    pub texture: Handle<Image>,
    pub starting_point: Point,
}

// Type components

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Minion;

// Motion components

#[derive(Component, Clone, Debug)]
pub enum EnemyMovementState {
    Stationary,
    Downward,
    Travel(Point),
    Seeking,
    CircleFormation(Formation),
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

#[derive(Debug, Clone, Component)]
pub struct Formation {
    pub radius: (f32, f32),
    pub pivot: Point,
    pub start: Point,
}

// Stat components

#[derive(Component, Clone, Debug, Default)]
pub struct EnemyStats {
    pub health: usize,
    pub spawn_rate: f32,  // from 0 to 1
    pub firing_rate: f32, // from 0 to 1
}

// AI components

#[derive(Component)]
pub struct EnemyAI {
    pub decision: EnemyDecision,
    pub reset_time: f32,
    pub timer: Timer,
}

pub enum EnemyDecision {
    None,
    Wander,
}

// Action nodes - success, failure, running
// Composite node - sequence nodes, executes all of the sequence nodes until one fails
// selector nodes, execute all nodes in a sequence until one of them succeeds
// for long running tasks create exclusive nodes that go until they finish running

