use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide, utils::HashSet};

use crate::stage_2_gameplay::components::{Invincibility, IsHit, IsHittable, SpriteSize};

use super::components::Asteroid;

pub fn asteroid_collision_system(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform, &SpriteSize), With<Asteroid>>,
    entity_query: Query<
        (Entity, &Transform, &SpriteSize),
        (With<IsHittable>, Without<Asteroid>, Without<Invincibility>),
    >,
) {
    let mut processed_entities: HashSet<Entity> = HashSet::new();

    for (asteroid_entity, asteroid_tf, asteroid_size) in asteroid_query.iter() {
        if processed_entities.contains(&asteroid_entity) {
            continue;
        }

        let asteroid_scale = asteroid_tf.scale.xy();

        for (entity, entity_tf, entity_size) in entity_query.iter() {
            if processed_entities.contains(&asteroid_entity) || processed_entities.contains(&entity)
            {
                continue;
            }

            let entity_scale = entity_tf.scale.xy();

            // determine if the collision has happened
            let collision = collide(
                asteroid_tf.translation,
                asteroid_size.0 * asteroid_scale,
                entity_tf.translation,
                entity_size.0 * entity_scale,
            );

            if collision.is_some() {
                // Adds hit to the entities so they are processed by other systems
                commands.entity(asteroid_entity).insert(IsHit);
                commands.entity(entity).insert(IsHit);

                // Adds entities to the hash set to not process them again
                processed_entities.insert(asteroid_entity);
                processed_entities.insert(entity);
            }
        }
    }
}
