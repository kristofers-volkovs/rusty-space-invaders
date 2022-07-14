use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    shared::resources::WinSize,
    stage_2_gameplay::{components::Point, constants::TIME_STEP},
};

use super::components::{MovementSpeed, SpawningDirection};

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

pub fn downwards_motion(current_point: Point, movement_speed: &MovementSpeed) -> Point {
    let distance = TIME_STEP * movement_speed.0;

    Point {
        x: current_point.x,
        y: current_point.y - distance,
    }
}
