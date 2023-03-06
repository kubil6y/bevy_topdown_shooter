use crate::prelude::*;
use bevy::prelude::*;

pub fn movement_system(
    mut query: Query<(Entity, &mut Transform, &Movable, &Velocity)>,
    time: Res<Time>,
) {
    for (_entity, mut tf, _movable, velocity) in query.iter_mut() {
        tf.translation.x += velocity.0.x * time.delta_seconds() * BASE_SPEED;
        tf.translation.y += velocity.0.y * time.delta_seconds() * BASE_SPEED;
    }
}
