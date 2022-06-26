#![allow(unused)] // temp

use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};
use components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Invincibility,
    InvincibilityTimer, Laser, Movable, Player, SpriteSize, Velocity,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

mod components;
mod enemy;
mod player;
mod ui;

// Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const PLAYER_HEART_FULL: &str = "hud_heartFull.png";
const PLAYER_HEART_EMPTY: &str = "hud_heartEmpty.png";

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

const SPRITE_SCALE: f32 = 0.5;

const GAME_FONT: &str = "MinimalPixel v2.ttf";

// Resources

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

struct UiTextures {
    heart_full: Handle<Image>,
    heart_empty: Handle<Image>,
    ui_font: Handle<Font>,
}

struct EnemyCount(u32);

struct PlayerState {
    on: bool,       // alive
    last_shot: f64, // -1 if not shot
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }

    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
    }
}

// Game Constants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const PLAYER_RESPAWN_DELAY: f64 = 2.;
const ENEMY_MAX: u32 = 2;
const FORMATION_MEMBERS_MAX: u32 = 2;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(UiPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .add_system(enemy_laser_hit_player_system)
        .add_system(invincibility_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window (for tutorial)
    window.set_position(IVec2::new(2780, 4900));

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

    let ui_textures = UiTextures {
        heart_full: asset_server.load(PLAYER_HEART_FULL),
        heart_empty: asset_server.load(PLAYER_HEART_EMPTY),
        ui_font: asset_server.load(GAME_FONT),
    };

    commands.insert_resource(game_textures);
    commands.insert_resource(ui_textures);
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
