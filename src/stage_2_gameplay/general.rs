use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use super::components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Invincibility,
    InvincibilityTimer, Laser, Movable, Player, SpriteSize, Velocity,
};
use super::constants::{
    BASE_SPEED, ENEMY_LASER_SPRITE, ENEMY_SPRITE, EXPLOSION_LEN, EXPLOSION_SHEET,
    PLAYER_LASER_SPRITE, PLAYER_SPRITE, TIME_STEP,
};
use super::enemy::formation::FormationMaker;
use super::resources::{EnemyCount, GameTextures, PlayerState};
use crate::shared::{
    constants::*,
    resources::{AppState, WinSize},
};

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .init_resource::<FormationMaker>()
            .add_enter_system(AppState::Gameplay, game_setup_system)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Gameplay)
                    .with_system(movable_system)
                    .with_system(player_laser_hit_enemy_system)
                    .with_system(explosion_to_spawn_system)
                    .with_system(explosion_animation_system)
                    .with_system(enemy_laser_hit_player_system)
                    .with_system(invincibility_system)
                    .into(),
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
    commands.insert_resource(EnemyCount(0));
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

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let mut despawn_entities: HashSet<Entity> = HashSet::new();

    // iterate over lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawn_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // iterate over enemies
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawn_entities.contains(&enemy_entity) || despawn_entities.contains(&laser_entity)
            {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // determine if laser and enemy collides
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // perform collision
            if let Some(_) = collision {
                // remove enemy
                commands.entity(enemy_entity).despawn();
                despawn_entities.insert(enemy_entity);
                enemy_count.0 -= 1;

                // remove laser
                commands.entity(laser_entity).despawn();
                despawn_entities.insert(laser_entity);

                // spawn the ExplosionToSpawn
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), (With<Player>, Without<Invincibility>)>,
) {
    if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::from(laser_tf.scale.xy());

            // determine if collided
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                player_tf.translation,
                player_size.0 * player_scale,
            );

            // perform collision
            if let Some(_) = collision {
                commands.entity(player_entity).despawn();
                player_state.shot(time.seconds_since_startup());

                // remove laser
                commands.entity(laser_entity).despawn();

                // spawn the explosionToSpawn
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(player_tf.translation.clone()));

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
    mut query: Query<(
        Entity,
        &mut InvincibilityTimer,
        &mut Invincibility,
        &mut Sprite,
    )>,
) {
    for (entity, mut timer, mut invincibility, mut sprite) in query.iter_mut() {
        invincibility.0 -= time.delta().as_secs_f32();
        timer.0.tick(time.delta());

        // entities alpha color is reset so it seems like the player is flickering
        if timer.0.finished() {
            let color_a = sprite.color.a();
            match sprite.color.a() {
                a if a == 1. => sprite.color.set_a(0.3),
                _ => sprite.color.set_a(1.),
            };
        }

        // when invincibility runs out then remove invincibility component and reset alpha for color
        if invincibility.0 < 0. {
            commands.entity(entity).remove::<Invincibility>();
            commands.entity(entity).remove::<InvincibilityTimer>();
            sprite.color.set_a(1.);
        }
    }
}
