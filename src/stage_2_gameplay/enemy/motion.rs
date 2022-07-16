use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    shared::resources::WinSize,
    stage_2_gameplay::{components::Point, constants::TIME_STEP, enemy::components::EnemyMovementState},
};

use super::components::{SpawningDirection, EnemyCount, EnemyMovement, Enemy};

pub fn calculate_spawning_point(spawn_direction: SpawningDirection, win_size: &WinSize) -> Point {
    let mut rng = thread_rng();

    let mut w_span = win_size.w / 2. + 100.;
    let h_span = win_size.h / 2. + 100.;

    match spawn_direction {
        SpawningDirection::Top => {
            w_span -= 100.;
            let x = rng.gen_range(-w_span..w_span) as f32;
            let y = h_span;
            Point { x, y }
        }
        SpawningDirection::Sides => {
            let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
            let y = rng.gen_range(-h_span..h_span) as f32;
            Point { x, y }
        }
    }
}


pub fn enemy_movement_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut enemy_count: ResMut<EnemyCount>,
    mut query: Query<(Entity, &mut Transform, &mut EnemyMovement), With<Enemy>>,
) {
    for (entity, mut transform, mut movement) in query.iter_mut() {
        // current position
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        // TODO add inertia and more fluid enemy movement
        let next_point: Point = match &movement.state {
            EnemyMovementState::Stationary => Point { x: x_org, y: y_org },
            EnemyMovementState::Downward => {
                let distance = TIME_STEP * movement.speed;
                Point {
                    x: x_org,
                    y: y_org - distance,
                }
            }
            EnemyMovementState::Travel(travel_point) => {
                let (x_end, y_end) = (travel_point.x, travel_point.y);

                // max distance in 1 sec
                let max_distance = TIME_STEP * movement.speed;

                // compute distance
                let dx = x_org - x_end;
                let dy = y_org - y_end;
                let distance = (dx * dx + dy * dy).sqrt();
                let distance_ratio = if distance != 0. {
                    max_distance / distance
                } else {
                    0.
                };

                // compute final x/y
                let x = x_org - dx * distance_ratio;
                let x = if dx > 0. { x.max(x_end) } else { x.min(x_end) };
                let y = y_org - dy * distance_ratio;
                let y = if dy > 0. { y.max(y_end) } else { y.min(y_end) };

                Point { x, y }
            }
            EnemyMovementState::Seeking => Point { x: x_org, y: y_org },
            EnemyMovementState::Circle(formation) => {
                // max distance in 1 sec
                let max_distance = (TIME_STEP / 4.) * movement.speed;

                // fixtures
                let dir: f32 = if formation.start.x < 0. { 1. } else { -1. }; // 1 for counter clockwise and -1 clockwise
                let (x_pivot, y_pivot) = (formation.pivot.x, formation.pivot.y);
                let (x_radius, y_radius) = formation.radius;

                // compute next angle
                let angle = movement.angle
                    + dir * movement.speed * TIME_STEP / (x_radius.min(y_radius) * PI / 2.);

                // compute target x/y
                let x_dst = x_radius * angle.cos() + x_pivot;
                let y_dst = y_radius * angle.sin() + y_pivot;

                // compute distance
                let dx = x_org - x_dst;
                let dy = y_org - y_dst;
                let distance = (dx * dx + dy * dy).sqrt();
                let distance_ratio = if distance != 0. {
                    max_distance / distance
                } else {
                    0.
                };

                // compute final x/y
                let x = x_org - dx * distance_ratio;
                let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
                let y = y_org - dy * distance_ratio;
                let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

                if distance < max_distance * movement.speed / 20. {
                    movement.angle = angle;
                }

                Point { x, y }
            },
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
            enemy_count.asteroids -= 1;
        }
    }
}

