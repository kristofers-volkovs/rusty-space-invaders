use bevy::prelude::*;


#[derive(Component, Clone, Debug)]
pub enum EnemyMovementState {
    Stationary,
    Downward,
    Travel,
    Seeking,
    Idle,
    Circle,
}

impl Default for EnemyMovementState {
    fn default() -> Self {
        EnemyMovementState::Idle
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct Health(pub usize);

impl From<usize> for Health {
    fn from(val: usize) -> Self {
        Health(val)
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct SpawnRate(pub f32);

impl From<f32> for SpawnRate {
    fn from(val: f32) -> Self {
        SpawnRate(val)
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct FiringRate(pub f32);

impl From<f32> for FiringRate {
    fn from(val: f32) -> Self {
        FiringRate(val)
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct MovementSpeed(pub f32);

impl From<f32> for MovementSpeed {
    fn from(val: f32) -> Self {
        MovementSpeed(val)
    }
}

#[derive(Bundle, Default, Clone, Debug)]
pub struct EnemyBundle {
    pub health: Health,
    pub spawn_rate: SpawnRate,   // from 0 to 1
    pub firing_rate: FiringRate, // from 0 to 1
    pub movement_speed: MovementSpeed,
    pub movement_state: EnemyMovementState,
}

pub enum SpawningDirection {
    Top,
    Sides,
}
