use crate::prelude::*;
use background::BackgroundPlugin;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod background;
mod constants;
mod enemy;
mod events;
mod player;
mod prelude;
mod resources;
mod shared;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Topdown Shooter".to_owned(),
                height: WINDOW_HEIGHT,
                width: WINDOW_WIDTH,
                ..default()
            },
            ..default()
        }))
        .add_startup_system_set_to_stage(
            StartupStage::PreStartup,
            SystemSet::new().with_system(setup),
        )
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(SharedPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // get window size as resource
    let window = windows.get_primary_mut().expect("no window :D?");
    let window_size = WindowSize::new(window.width(), window.height());
    commands.insert_resource(window_size);

    // create explosion texture atlas
    let texture_handle = asset_server.load(SPRITE_SHEET_EXPLOSION);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64., 64.),
        4,
        4,
        None,
        None,
    );
    let explosion = texture_atlases.add(texture_atlas);

    // load game textures
    let game_textures = GameTextures {
        player: asset_server.load(SPRITE_PLAYER_SHIP),
        laser_player: asset_server.load(SPRITE_LASER_PLAYER),
        laser_enemy: asset_server.load(SPRITE_LASER_ENEMY),
        enemy: asset_server.load(SPRITE_ENEMY_SHIP),
        background: asset_server.load(SPRITE_BACKGROUND),
        explosion,
    };
    commands.insert_resource(game_textures);

    // load game audio sources
    let audio_assets = AudioAssets {
        player_shoot: asset_server.load(AUDIO_PLAYER_SHOOT),
        enemy_shoot: asset_server.load(AUDIO_ENEMY_SHOOT),
        hit: asset_server.load(AUDIO_HIT),
        explosion: asset_server.load(AUDIO_EXPLOSION),
        death: asset_server.load(AUDIO_DEATH),
        powerup: asset_server.load(AUDIO_POWERUP),
        gold: asset_server.load(AUDIO_GOLD),
        theme_song: asset_server.load(THEME_SONG),
    };
    commands.insert_resource(audio_assets);
}
