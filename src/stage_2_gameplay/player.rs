use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::{
    AppLooplessStateExt, ConditionHelpers, ConditionSet, FixedTimestepStage, IntoConditionalSystem,
};

use super::components::{
    DespawnEntity, EntityType, ExplosionToSpawn, FromEntity, IsHit, IsHittable,
};
use super::constants::{
    HIT_DETECTION, HIT_PROCESSING, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE,
    SPRITE_SCALE,
};
use super::resources::{GameTextures, PlayerState};
use crate::shared::components::{GameRunning, SpawnPlayer};
use crate::shared::resources::{AppState, WinSize};
use crate::stage_2_gameplay::components::{
    FiringCooldownTimer, Invincibility, Laser, Movable, Player, SpriteSize, Velocity,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            player_spawn_system
                .run_in_state(AppState::Gameplay)
                .run_if_resource_added::<SpawnPlayer>(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Gameplay)
                .with_system(player_keyboard_event_system)
                .with_system(player_fire_system)
                .with_system(firing_cooldown_system)
                .into(),
        )
        .add_system(
            player_hit_system
                .run_in_state(AppState::Gameplay)
                .label(HIT_PROCESSING)
                .after(HIT_DETECTION),
        );
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    // add player
    let bottom = -win_size.h / 2.; // bottom of the screen
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(EntityType::Player)
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(IsHittable)
        .insert(Velocity { x: 0., y: 0. })
        .insert(Invincibility::from(3.));

    commands.remove_resource::<SpawnPlayer>()
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &Transform), (With<Player>, Without<FiringCooldownTimer>)>,
) {
    if let Ok((player_entity, player_tf)) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 15., 0.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. })
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(FromEntity::FromPlayer)
                    .insert(Laser);
            };

            spawn_laser(x_offset);
            spawn_laser(-x_offset);

            commands
                .entity(player_entity)
                .insert(FiringCooldownTimer::default());
        }
    }
}

fn player_hit_system(
    mut commands: Commands,
    mut ev_despawn: EventWriter<DespawnEntity>,
    mut player_state: ResMut<PlayerState>,
    query: Query<(Entity, &Transform, &EntityType), (With<Player>, With<IsHit>)>,
) {
    if let Ok((entity, entity_tf, entity_type)) = query.get_single() {
        player_state.shot();

        if player_state.health == 0 {
            ev_despawn.send(DespawnEntity {
                entity: entity,
                entity_type: entity_type.clone(),
            });

            commands
                .spawn()
                .insert(ExplosionToSpawn(entity_tf.translation));
        } else {
            commands.entity(entity).remove::<IsHit>();
            commands.entity(entity).insert(Invincibility::from(3.));
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) || kb.pressed(KeyCode::A) {
            -1.
        } else if kb.pressed(KeyCode::Right) || kb.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };
    }
}

fn firing_cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FiringCooldownTimer)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            commands.entity(entity).remove::<FiringCooldownTimer>();
        }
    }
}
