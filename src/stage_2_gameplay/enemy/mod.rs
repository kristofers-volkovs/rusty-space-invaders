use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::{ConditionSet, FixedTimestepStage};
use rand::{thread_rng, Rng};

use super::constants::{
    BASE_SPEED, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE, ENEMY_SPAWN, SPRITE_SCALE, TIME_STEP,
};
use super::resources::{EnemyCount, GameTextures};
use crate::shared::resources::{AppState, WinSize};
use crate::stage_2_gameplay::components::{
    Enemy, FromEnemy, Laser, Movable, Point, SpriteSize, Velocity,
};

use self::components::{
    EnemyBundle, EnemyMovementState, Health, MovementSpeed, SpawnRate, SpawningDirection,
};
use self::formation::{Formation, FormationMaker};
use self::motion::{calculate_spawning_point, downwards_motion};

pub mod components;
pub mod formation;
pub mod motion;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        let mut fixedupdate = SystemStage::parallel();
        fixedupdate.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Gameplay)
                .with_system(enemy_spawn_system)
                .into(),
        );

        app.add_stage_before(
            CoreStage::Update,
            ENEMY_SPAWN,
            FixedTimestepStage::from_stage(Duration::from_secs_f32(0.5), fixedupdate),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Gameplay)
                .with_system(enemy_movement_system)
                .with_system(enemy_fire_system)
                .into(),
        );
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        let starting_point = calculate_spawning_point(SpawningDirection::Top, &win_size);
        let (x, y) = (starting_point.x, starting_point.y);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(EnemyBundle {
                movement_speed: MovementSpeed::from(BASE_SPEED),
                movement_state: EnemyMovementState::Downward,
                ..Default::default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &tf in enemy_query.iter() {
        if thread_rng().gen_bool(1. / 60.) {
            let (x, y) = (tf.translation.x, tf.translation.y);

            // spawn enemy laser
            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.enemy_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y - 15., 0.),
                        rotation: Quat::from_rotation_x(PI),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(SpriteSize::from(ENEMY_LASER_SIZE))
                .insert(FromEnemy)
                .insert(Movable { auto_despawn: true })
                .insert(Velocity { x: 0., y: -1. });
        }
    }
}

fn enemy_movement_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut enemy_count: ResMut<EnemyCount>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &EnemyMovementState, &MovementSpeed), With<Enemy>>,
) {
    for (entity, mut transform, movement_state, movement_speed) in query.iter_mut() {
        // current position
        let current_point = Point {
            x: transform.translation.x,
            y: transform.translation.y,
        };

        let next_point: Point = match movement_state {
            EnemyMovementState::Stationary => current_point.clone(),
            EnemyMovementState::Downward => downwards_motion(current_point, movement_speed),
            EnemyMovementState::Travel => current_point.clone(),
            EnemyMovementState::Seeking => current_point.clone(),
            EnemyMovementState::Idle => current_point.clone(),
            EnemyMovementState::Circle => current_point.clone(),
        };

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (next_point.x, next_point.y);

        // TODO create a more universal way to despawn enemies when it's outside the window
        const MARGIN: f32 = 200.;
        if translation.y > win_size.h / 2. + MARGIN
            || translation.y < -win_size.h / 2. - MARGIN
            || translation.x > win_size.w / 2. + MARGIN
            || translation.x < -win_size.w / 2. - MARGIN
        {
            commands.entity(entity).despawn();
            enemy_count.0 -= 1;
        }
    }
}
