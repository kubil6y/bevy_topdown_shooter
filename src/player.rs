use crate::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system)
            .add_event::<PlayerLaserFireEvent>()
            .add_system(player_movement_system)
            .add_system(player_input_system)
            .add_system(spawn_player_laser_system);
    }
}

fn spawn_player_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
) {
    let (px, py) = (0., -window_size.height * 3. / 4.);
    // spawn player
    commands.spawn((
        SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform::from_translation(Vec3::new(px, py, 99.)),
            ..default()
        },
        Player,
        //Movable::default(),
        Velocity::default(),
    ));
}

fn player_input_system(
    keyboard: Res<Input<KeyCode>>,
    mut laser_fire_event: EventWriter<PlayerLaserFireEvent>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
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
            -window_size.height / 2. + SIZE_PLAYER_SHIP.1 / 2.,
            -window_size.height / 4.,
        );

        // handle horizontal movement
        tf.translation.x += velocity.0.x * BASE_SPEED * time.delta_seconds();
        if tf.translation.x - SIZE_PLAYER_SHIP.0 >= window_size.width / 2. {
            tf.translation.x = -window_size.width / 2.;
        }
        if tf.translation.x + SIZE_PLAYER_SHIP.0 <= -window_size.width / 2. {
            tf.translation.x = window_size.width / 2.;
        }
    }
}

fn spawn_player_laser_system(mut events: EventReader<PlayerLaserFireEvent>) {
    for event in events.iter() {
        println!("laser fired {:?}", event);
    }
}
