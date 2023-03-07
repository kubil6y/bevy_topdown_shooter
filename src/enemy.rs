use std::collections::HashSet;

use crate::prelude::*;
use bevy::{prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyCount(0))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.))
                    .with_system(spawn_enemy_system),
            )
            .add_system(spawn_enemy_laser_system)
            .add_system(handle_enemy_out_of_bounds_system)
            .add_system(handle_enemy_take_hit_system)
            .add_event::<EnemyLaserFireEvent>()
            .add_event::<EnemyTakeHitEvent>()
            .add_event::<PlayerTakeHitEvent>();
    }
}

fn spawn_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
) {
    if enemy_count.0 >= MAX_ENEMY_COUNT {
        return;
    }

    // generate random position for enemy
    let mut rng = thread_rng();
    let enemy_x = rng.gen_range(
        (-window_size.width / 2. + SIZE_ENEMY_SHIP.0 / 2.)
            ..(window_size.width / 2. - SIZE_ENEMY_SHIP.0 / 2.),
    );
    let enemy_y = (rng.gen_range(10..50) as f32) + window_size.height / 2.;

    commands.spawn((
        SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                translation: Vec3::new(enemy_x, enemy_y, 1.),
                scale: Vec3::new(SPRITE_SCALE, -SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        },
        Enemy,
        Collision::from(SIZE_ENEMY_SHIP),
        Movable { auto_despawn: true },
        Velocity(Vec2::new(0., -0.1)), //  -.1
    ));

    enemy_count.0 += 1;
}

fn spawn_enemy_laser_system() {}

fn handle_enemy_out_of_bounds_system(
    mut commands: Commands,
    mut out_of_bounds: EventWriter<PlayerTakeHitEvent>,
    mut enemy_count: ResMut<EnemyCount>,
    query: Query<(Entity, &Transform), With<Enemy>>,
    window_size: Res<WindowSize>,
) {
    for (entity, tf) in query.iter() {
        if tf.translation.y + SIZE_ENEMY_SHIP.1 / 2. < -window_size.height / 2.
        {
            out_of_bounds.send_default();
            commands.entity(entity).despawn_recursive();
            enemy_count.0 -= 1;
        }
    }
}

fn handle_enemy_take_hit_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    mut take_hit_events: EventReader<EnemyTakeHitEvent>,
    mut explosion_event: EventWriter<ExplosionEvent>,
) {
    let mut despawned: HashSet<Entity> = HashSet::new();
    for event in take_hit_events.iter() {
        if despawned.contains(&event.0) {
            continue;
        }
        explosion_event.send(ExplosionEvent(Vec2::new(event.1.x, event.1.y)));
        commands.entity(event.0).despawn_recursive();
        enemy_count.0 -= 1;
        despawned.insert(event.0);
    }
}
