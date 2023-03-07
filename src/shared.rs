use crate::prelude::*;
use bevy::prelude::*;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system)
            .add_system(auto_despawner_system);
    }
}

pub fn movement_system(
    mut query: Query<(Entity, &mut Transform, &Movable, &Velocity)>,
    time: Res<Time>,
) {
    for (_entity, mut tf, _movable, velocity) in query.iter_mut() {
        tf.translation.x += velocity.0.x * time.delta_seconds() * BASE_SPEED;
        tf.translation.y += velocity.0.y * time.delta_seconds() * BASE_SPEED;
    }
}

pub fn auto_despawner_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Collision, &Movable)>,
    window_size: Res<WindowSize>,
) {
    for (entity, tf, collision, movable) in query.iter() {
        if movable.auto_despawn {
            if check_out_of_bounds(
                tf.translation,
                collision.0,
                Vec2::new(window_size.width, window_size.height),
            ) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn check_out_of_bounds(
    translation: Vec3,
    box_size: Vec2,
    window_size: Vec2,
) -> bool {
    if translation.x + box_size.x / 2. > window_size.x / 2.
        || translation.x - box_size.x / 2. < -window_size.x / 2.
        || translation.y + box_size.y / 2. > window_size.y / 2.
        || translation.y - box_size.y / 2. < -window_size.y / 2.
    {
        true
    } else {
        false
    }
}
