use bevy::ecs::system::Resource;
use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};
use iyes_loopless::prelude::{
    AppLooplessStateExt, ConditionHelpers, ConditionSet, IntoConditionalSystem,
};
use iyes_loopless::state::NextState;

use super::components::{
    DespawnEntity, EntityType, Explosion, ExplosionTimer, ExplosionToSpawn, FromEntity,
    Invincibility, IsHit, IsHittable, Laser, Movable, Player, SpriteSize, Velocity,
};
use super::constants::{
    BASE_SPEED, ENEMY_LASER_SPRITE, ENEMY_SPRITE, EXPLOSION_LEN, EXPLOSION_SHEET, GAMEPLAY_RESET,
    HIT_DETECTION, HIT_PROCESSING, PLAYER_LASER_SPRITE, PLAYER_SPRITE, TIME_STEP,
};
use super::enemy::components::{Enemy, EnemyCount};
use super::enemy::formation::FormationMaker;
use super::resources::{GameTextures, PlayerState};
use crate::shared::components::{GameRunning, ResetGameplay, SpawnPlayer};
use crate::shared::general::despawn_system;
use crate::shared::{
    constants::*,
    resources::{AppState, WinSize},
};

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnEntity>()
            .add_enter_system_set(
                AppState::Gameplay,
                SystemSet::new()
                    .with_system(game_setup_system.run_unless_resource_exists::<GameRunning>())
                    .with_system(
                        init_game_resource_system.run_unless_resource_exists::<GameRunning>(),
                    ),
            )
            // --- Main gameplay loop ---
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Gameplay)
                    .with_system(movable_system)
                    .with_system(explosion_to_spawn_system)
                    .with_system(explosion_animation_system)
                    .with_system(invincibility_system)
                    .into(),
            )
            .add_system(
                laser_hit_system
                    .run_in_state(AppState::Gameplay)
                    .label(HIT_DETECTION),
            )
            .add_system(
                entity_despawn_system
                    .run_in_state(AppState::Gameplay)
                    .after(HIT_PROCESSING),
            )
            // --- Despawns the mobs and resets resources ---
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Gameplay)
                    .run_if_resource_exists::<ResetGameplay>()
                    .label(GAMEPLAY_RESET)
                    // Despawns everyone on the board
                    .with_system(despawn_system::<Enemy>)
                    .with_system(despawn_system::<Player>)
                    .with_system(despawn_system::<Laser>)
                    // Reinitiates resources
                    .with_system(init_game_resource_system)
                    .into(),
            )
            .add_system(
                remove_resource::<ResetGameplay>
                    .run_if_resource_exists::<ResetGameplay>()
                    .after(GAMEPLAY_RESET),
            );
    }
}

fn game_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // create explosion texture atlas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    let explosion = texture_atlases.add(texture_atlas);

    // add GameTexture resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };

    commands.insert_resource(game_textures);
    commands.insert_resource(SpawnPlayer);
}

fn init_game_resource_system(mut commands: Commands) {
    commands.insert_resource(EnemyCount::default());
    commands.insert_resource(PlayerState::default());
    commands.insert_resource(GameRunning);
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn entity_despawn_system(
    mut commands: Commands,
    mut ev_despawn: EventReader<DespawnEntity>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    for ev in ev_despawn.iter() {
        commands.entity(ev.entity).despawn();

        match ev.entity_type {
            EntityType::Asteroid => {
                enemy_count.asteroids -= 1;
            }
            EntityType::Minion => {
                enemy_count.minions -= 1;
            }
            _ => {}
        }
    }
}

fn laser_hit_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize, &FromEntity), With<Laser>>,
    entity_query: Query<
        (Entity, &Transform, &SpriteSize, &EntityType),
        (With<IsHittable>, Without<Invincibility>),
    >,
) {
    let mut processed_entities: HashSet<Entity> = HashSet::new();

    for (entity, entity_tf, entity_size, entity_type) in entity_query.iter() {
        if processed_entities.contains(&entity) {
            continue;
        }

        let entity_scale = entity_tf.scale.xy();

        for (laser_entity, laser_tf, laser_size, from_entity) in laser_query.iter() {
            // if entity is player and it's a player laser then skip
            // same if enemy entity and enemy laser
            match entity_type {
                EntityType::Player => {
                    if let FromEntity::FromPlayer = from_entity {
                        continue;
                    }
                }
                _ => {
                    if let FromEntity::FromEnemy = from_entity {
                        continue;
                    }
                }
            };

            if processed_entities.contains(&entity) || processed_entities.contains(&laser_entity) {
                continue;
            }

            let laser_scale = laser_tf.scale.xy();

            // determine if the collision has happened
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                entity_tf.translation,
                entity_size.0 * entity_scale,
            );

            // laser has collided with the entity
            if collision.is_some() {
                // remove laser
                commands.entity(laser_entity).despawn();
                processed_entities.insert(laser_entity);

                // Add hit to entity so that another system processes it
                commands.entity(entity).insert(IsHit);
                processed_entities.insert(entity);

                break;
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        // spawn explosion sprite
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        // despawn the explosionToSpawn
        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn invincibility_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Invincibility, &mut Sprite)>,
) {
    for (entity, mut invincibility, mut sprite) in query.iter_mut() {
        invincibility.length -= time.delta().as_secs_f32();
        invincibility.animation_timer.tick(time.delta());

        // entities alpha color is reset so it seems like the player is flickering
        if invincibility.animation_timer.finished() {
            let color_a = sprite.color.a();
            match sprite.color.a() {
                a if a == 1. => sprite.color.set_a(0.3),
                _ => sprite.color.set_a(1.),
            };
        }

        // when invincibility runs out then remove invincibility component and reset alpha for color
        if invincibility.length < 0. {
            commands.entity(entity).remove::<Invincibility>();
            sprite.color.set_a(1.);
        }
    }
}

fn remove_resource<R: Resource>(mut commands: Commands) {
    commands.remove_resource::<R>();
}
