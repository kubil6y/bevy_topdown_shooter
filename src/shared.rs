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
    query: Query<(Entity, &Transform, &Collision, &Movable), Without<Enemy>>,
    window_size: Res<WindowSize>,
) {
    for (entity, tf, collision, movable) in query.iter() {
        if movable.auto_despawn
            && (tf.translation.x + collision.0.x / 2. > window_size.width / 2.
                || tf.translation.x - collision.0.x / 2.
                    < -window_size.width / 2.
                || tf.translation.y + collision.0.y / 2.
                    > window_size.height / 2.
                || tf.translation.y + collision.0.y / 2.
                    < -window_size.height / 2.)
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}
