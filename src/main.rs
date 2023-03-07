use crate::prelude::*;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
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
                height: 600.,
                width: 400.,
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
        .add_plugin(SharedPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(GameState::default());

    // get window size as resource
    let window = windows.get_primary_mut().expect("no window :D?");
    let window_size = WindowSize::new(window.width(), window.height());
    commands.insert_resource(window_size);

    // load game textures
    let game_textures = GameTextures {
        player: asset_server.load(SPRITE_PLAYER_SHIP),
        laser_player: asset_server.load(SPRITE_LASER_PLAYER),
        laser_enemy: asset_server.load(SPRITE_LASER_ENEMY),
        enemy: asset_server.load(SPRITE_ENEMY_SHIP),
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
    };
    commands.insert_resource(audio_assets);
}
