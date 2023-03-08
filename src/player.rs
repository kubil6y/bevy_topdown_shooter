use std::collections::HashSet;

use crate::prelude::*;
use bevy::{prelude::*, sprite::collide_aabb::collide, time::FixedTimestep};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.))
                    .with_system(spawn_player_system),
            )
            .add_system(player_movement_system)
            .add_system(player_input_system)
            .add_system(spawn_player_laser_system)
            .add_system(player_laser_hit_enemies)
            .add_system(player_enemy_collision_system)
            .add_system(handle_player_take_hit_event)
            .add_system(handle_wave_complete_event_system)
            .add_system(handle_player_death_event_system)
            .add_event::<WaveCompleteEvent>()
            .add_event::<PlayerLaserFireEvent>()
            .add_event::<PlayerDeathEvent>();
    }
}

fn spawn_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
) {
    if player_state.is_alive {
        return;
    }

    let (px, py) = (0., -window_size.height * 1. / 4.);
    commands.spawn((
        SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(px, py, 99.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        },
        Player,
        Collision::from(SIZE_PLAYER_SHIP),
        Velocity::default(),
    ));

    player_state.spawn();
}

fn player_input_system(
    player_state: Res<PlayerState>,
    keyboard: Res<Input<KeyCode>>,
    mut laser_fire_event: EventWriter<PlayerLaserFireEvent>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    if !player_state.is_alive {
        return;
    }

    if let Ok((mut velocity, tf)) = query.get_single_mut() {
        velocity.0.x = if keyboard.pressed(KeyCode::S)
            || keyboard.pressed(KeyCode::Left)
        {
            -1.
        } else if keyboard.pressed(KeyCode::F)
            || keyboard.pressed(KeyCode::Right)
        {
            1.
        } else {
            0.
        };
        velocity.0.y =
            if keyboard.pressed(KeyCode::E) || keyboard.pressed(KeyCode::Up) {
                1.
            } else if keyboard.pressed(KeyCode::D)
                || keyboard.pressed(KeyCode::Down)
            {
                -1.
            } else {
                0.
            };
        if keyboard.just_pressed(KeyCode::Space) {
            let offset = Vec2::new(
                tf.translation.x,
                tf.translation.y + SIZE_PLAYER_SHIP.1 / 2.,
            );
            laser_fire_event.send(PlayerLaserFireEvent(offset));
        }
    }
}

fn player_movement_system(
    window_size: Res<WindowSize>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    player_state: Res<PlayerState>,
    time: Res<Time>,
) {
    for (mut tf, velocity) in query.iter_mut() {
        // handle vertical movement
        //let new_y =
        //tf.translation.y + velocity.0.y * time.delta_seconds() * BASE_SPEED;

        let new_y = tf.translation.y
            + velocity.0.y * time.delta_seconds() * player_state.speed;
        tf.translation.y = f32::clamp(
            new_y,
            -window_size.height / 2. + SIZE_PLAYER_SHIP.1 / 2. * SPRITE_SCALE,
            -window_size.height / 6.,
        );

        // handle horizontal movement
        //tf.translation.x += velocity.0.x * BASE_SPEED * time.delta_seconds();
        tf.translation.x +=
            velocity.0.x * player_state.speed * time.delta_seconds();
        if tf.translation.x - SIZE_PLAYER_SHIP.0 * SPRITE_SCALE
            >= window_size.width / 2.
        {
            tf.translation.x = -window_size.width / 2.;
        }
        if tf.translation.x + SIZE_PLAYER_SHIP.0 * SPRITE_SCALE
            <= -window_size.width / 2.
        {
            tf.translation.x = window_size.width / 2.;
        }
    }
}

fn spawn_player_laser_system(
    mut commands: Commands,
    mut player_laser_fire_events: EventReader<PlayerLaserFireEvent>,
    audio_assets: Res<AudioAssets>,
    game_textures: Res<GameTextures>,
    audio: Res<Audio>,
) {
    for event in player_laser_fire_events.iter() {
        audio.play(audio_assets.player_shoot.clone());
        let (laser_x, laser_y) = (
            event.0.x,
            event.0.y + (SIZE_PLAYER_SHIP.1 / 2. * SPRITE_SCALE) + 1.,
        );
        commands.spawn((
            SpriteBundle {
                texture: game_textures.laser_player.clone(),
                transform: Transform {
                    translation: Vec3::new(laser_x, laser_y, 1.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            },
            Laser,
            FromPlayer,
            Movable { auto_despawn: true },
            Collision::from(SIZE_LASER_PLAYER),
            Velocity(Vec2::new(0., 1.5)),
        ));
    }
}

fn player_laser_hit_enemies(
    mut commands: Commands,
    mut hit_enemy_event: EventWriter<EnemyTakeHitEvent>,
    query_player_laser: Query<
        (Entity, &Transform, &Collision),
        (With<Laser>, With<FromPlayer>),
    >,
    query_enemies: Query<(Entity, &Transform, &Collision), With<Enemy>>,
) {
    let mut despawned: HashSet<Entity> = HashSet::new();
    for (pl_entity, pl_tf, pl_size) in query_player_laser.iter() {
        if despawned.contains(&pl_entity) {
            continue;
        }
        for (e_entity, e_tf, e_size) in query_enemies.iter() {
            if despawned.contains(&e_entity) || despawned.contains(&pl_entity) {
                continue;
            }

            let collision = collide(
                pl_tf.translation,
                pl_size.0,
                e_tf.translation,
                e_size.0,
            );
            if collision.is_some() {
                commands.entity(pl_entity).despawn_recursive();
                despawned.insert(pl_entity);
                despawned.insert(e_entity);
                hit_enemy_event
                    .send(EnemyTakeHitEvent(e_entity, e_tf.translation));
            }
        }
    }
}

fn player_enemy_collision_system(
    mut player_take_hit_event: EventWriter<PlayerTakeHitEvent>,
    mut enemy_take_hit_event: EventWriter<EnemyTakeHitEvent>,
    query_enemies: Query<(Entity, &Transform, &Collision), With<Enemy>>,
    query_player: Query<(&Transform, &Collision), With<Player>>,
) {
    let player = query_player.get_single();
    if let Ok((player_tf, player_size)) = player {
        for (enemy_entity, enemy_tf, enemy_size) in query_enemies.iter() {
            let collision = collide(
                player_tf.translation,
                player_size.0,
                enemy_tf.translation,
                enemy_size.0,
            );
            if collision.is_some() {
                player_take_hit_event.send_default();
                enemy_take_hit_event.send(EnemyTakeHitEvent(
                    enemy_entity,
                    enemy_tf.translation,
                ));
            }
        }
    }
}

fn handle_player_take_hit_event(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    mut enemy_attrs: ResMut<EnemyAttributes>,
    mut take_hit_events: EventReader<PlayerTakeHitEvent>,
    mut explosion_event: EventWriter<ExplosionEvent>,
    mut player_death_event: EventWriter<PlayerDeathEvent>,
    mut query: Query<(Entity, &mut Transform), With<Player>>,
    window_size: Res<WindowSize>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for _ in take_hit_events.iter() {
        player_state.decrement_health();

        if player_state.is_alive {
            audio.play(audio_assets.hit.clone());
        } else {
            if !player_state.death_sound_played {
                audio.play(audio_assets.death.clone());
                player_state.death_sound_played = true;
                if let Ok((entity, mut tf)) = query.get_single_mut() {
                    explosion_event.send(ExplosionEvent {
                        position: Vec2::new(tf.translation.x, tf.translation.y),
                        with_sound: true,
                    });

                    let (px, py) = (0., -window_size.height * 1. / 4.);
                    tf.translation = Vec3::new(px, py, 99.);
                    commands.entity(entity).despawn_recursive();
                    player_state.is_alive = false;
                    enemy_attrs.reset();
                    player_death_event.send_default();
                }
            }
        }
    }
}
fn handle_wave_complete_event_system(
    mut events: EventReader<WaveCompleteEvent>,
    mut enemy_attrs: ResMut<EnemyAttributes>,
    mut player_state: ResMut<PlayerState>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for _ in events.iter() {
        enemy_attrs.upgrade();
        player_state.upgrade();
        audio.play(audio_assets.powerup.clone());
    }
}

fn handle_player_death_event_system(
    mut commands: Commands,
    mut events: EventReader<PlayerDeathEvent>,
    game_textures: Res<GameTextures>,
    query_lasers: Query<Entity, With<Laser>>,
    query_enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for _ in events.iter() {
        let mut entities: HashSet<Entity> = HashSet::new();
        let mut explosions: Vec<Vec3> = Vec::with_capacity(24);

        for (enemy_entity, enemy_tf) in query_enemies.iter() {
            explosions.push(enemy_tf.translation);
            entities.insert(enemy_entity);
        }

        for laser_entity in query_lasers.iter() {
            entities.insert(laser_entity);
        }

        for explosion in explosions.iter() {
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: game_textures.explosion.clone(),
                    transform: Transform {
                        translation: Vec3::new(explosion.x, explosion.y, 1.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..default()
                    },
                    ..default()
                },
                Explosion(Timer::from_seconds(0.05, TimerMode::Once)),
            ));
        }

        for entity in entities {
            commands.entity(entity).despawn_recursive();
        }
    }
}
