use crate::prelude::*;
use bevy::{prelude::*, utils::HashSet};

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(play_theme_song)
            .add_startup_system(spawn_simple_ui_system)
            .add_system(update_simple_ui_system)
            .add_system(movement_system)
            .add_system(auto_despawner_system)
            .add_system(handle_explosion_event_system)
            .add_system(explosion_animation_system)
            .add_event::<ExplosionEvent>();
    }
}

fn movement_system(
    mut query: Query<(Entity, &mut Transform, &Movable, &Velocity)>,
    time: Res<Time>,
) {
    for (_entity, mut tf, _movable, velocity) in query.iter_mut() {
        tf.translation.x += velocity.0.x * time.delta_seconds() * BASE_SPEED;
        tf.translation.y += velocity.0.y * time.delta_seconds() * BASE_SPEED;
    }
}

fn auto_despawner_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Collision, &Movable), Without<Enemy>>,
    window_size: Res<WindowSize>,
) {
    let mut despawned: HashSet<Entity> = HashSet::new();
    for (entity, tf, collision, movable) in query.iter() {
        if despawned.contains(&entity) {
            continue;
        }
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
            despawned.insert(entity);
        }
    }
}

fn handle_explosion_event_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut explosion_events: EventReader<ExplosionEvent>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for event in explosion_events.iter() {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        event.position.x,
                        event.position.y,
                        1.,
                    ),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            },
            Explosion(Timer::from_seconds(0.05, TimerMode::Once)),
        ));
        audio.play(audio_assets.explosion.clone());
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &mut Explosion, &mut TextureAtlasSprite),
        With<Explosion>,
    >,
) {
    for (entity, mut explosion_timer, mut sprite) in query.iter_mut() {
        explosion_timer.0.tick(time.delta());
        if explosion_timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= EXPLOSION_LENGTH {
                commands.entity(entity).despawn_recursive()
            }
        }
    }
}

fn play_theme_song(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_with_settings(
        audio_assets.theme_song.clone(),
        PlaybackSettings {
            repeat: true,
            volume: 0.331488,
            ..default()
        },
    );
}

fn spawn_simple_ui_system(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "HP: 0\nSCORE: 0\nLevel: 0",
            TextStyle {
                font: font_assets.ui.clone(),
                font_size: 20.,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::CENTER_LEFT)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        }),
        UIFont,
    ));
}

fn update_simple_ui_system(
    mut query: Query<&mut Text, With<UIFont>>,
    player_state: Res<PlayerState>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!(
            "HP: {}\nSCORE: {}\nLevel: {}",
            player_state.health, player_state.score, player_state.golds
        );
    }
}
