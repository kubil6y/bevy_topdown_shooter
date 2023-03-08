use crate::prelude::*;
use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BackgroundSpeed(BACKGROUND_BASE_SPEED))
            .add_startup_system(spawn_background_system)
            .add_system(bg_movement_system)
            .add_system(handle_wave_complete_event);
    }
}

fn spawn_background_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    commands.spawn((
        SpriteBundle {
            texture: game_textures.background.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        },
        Background,
        Velocity(Vec2::new(0., -1.)),
    ));

    commands.spawn((
        SpriteBundle {
            texture: game_textures.background.clone(),
            transform: Transform {
                translation: Vec3::new(0., SIZE_BACKGROUND.1, 0.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        },
        Background,
        Velocity(Vec2::new(0., -1.)),
    ));
}

fn bg_movement_system(
    mut query: Query<(&mut Transform, &Velocity), With<Background>>,
    background_speed: Res<BackgroundSpeed>,
    time: Res<Time>,
) {
    for (mut tf, velocity) in query.iter_mut() {
        tf.translation.y +=
            velocity.0.y * time.delta_seconds() * background_speed.0;

        if tf.translation.y - 20. < -SPRITE_SCALE * SIZE_BACKGROUND.1 {
            tf.translation.y = SPRITE_SCALE * SIZE_BACKGROUND.1;
        }
    }
}

fn handle_wave_complete_event(
    mut events: EventReader<WaveCompleteEvent>,
    mut background_speed: ResMut<BackgroundSpeed>,
) {
    for _ in events.iter() {
        background_speed.0 += 20.;
    }
}
