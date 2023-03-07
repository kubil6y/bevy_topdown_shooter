use crate::prelude::*;
use bevy::{prelude::*, time::FixedTimestep};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(spawn_player_system),
            )
            .add_system(player_movement_system)
            .add_system(player_input_system)
            .add_system(spawn_player_laser_system)
            .add_system(handle_player_take_hit_event)
            .add_system(log_player_state)
            .add_event::<PlayerLaserFireEvent>();
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
        velocity.0.x = if keyboard.pressed(KeyCode::S) {
            -1.
        } else if keyboard.pressed(KeyCode::F) {
            1.
        } else {
            0.
        };
        velocity.0.y = if keyboard.pressed(KeyCode::E) {
            1.
        } else if keyboard.pressed(KeyCode::D) {
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
    time: Res<Time>,
) {
    for (mut tf, velocity) in query.iter_mut() {
        // handle vertical movement
        let new_y =
            tf.translation.y + velocity.0.y * time.delta_seconds() * BASE_SPEED;
        tf.translation.y = f32::clamp(
            new_y,
            -window_size.height / 2. + SIZE_PLAYER_SHIP.1 / 2. * SPRITE_SCALE,
            -window_size.height / 6.,
        );

        // handle horizontal movement
        tf.translation.x += velocity.0.x * BASE_SPEED * time.delta_seconds();
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
    mut events: EventReader<PlayerLaserFireEvent>,
    audio_assets: Res<AudioAssets>,
    game_textures: Res<GameTextures>,
    audio: Res<Audio>,
) {
    for event in events.iter() {
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

fn handle_player_take_hit_event(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    mut take_hit_events: EventReader<PlayerTakeHitEvent>,
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
                    let (px, py) = (0., -window_size.height * 1. / 4.);
                    tf.translation = Vec3::new(px, py, 99.);
                    commands.entity(entity).despawn_recursive();
                    player_state.is_alive = false;
                }
            }
        }
    }
}

fn log_player_state(player_state: Res<PlayerState>) {
    println!("{:?}", player_state); // TODO
}
