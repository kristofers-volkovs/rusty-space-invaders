use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{stage_2_gameplay::{
    components::{
        DespawnEntity, EntityType, ExplosionToSpawn, FromEntity, IsHit, Laser, Movable, SpriteSize,
        Velocity,
    },
    constants::{ENEMY_LASER_SIZE, SPRITE_SCALE},
    resources::GameTextures,
}, shared::resources::WinSize};

use super::components::{EnemyStats, Minion};

pub fn minion_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Minion>>,
) {
    for &tf in enemy_query.iter() {
        if thread_rng().gen_bool(1. / 60.) {
            let (x, y) = (tf.translation.x, tf.translation.y);

            // spawn minion laser
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
                .insert(FromEntity::FromEnemy)
                .insert(Movable { auto_despawn: true })
                .insert(Velocity { x: 0., y: -1. });
        }
    }
}

pub fn minion_navigation_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<&mut Transform, With<Minion>>,
) {
    for (mut tf) in query.iter_mut() {
        println!("");
    }
}
