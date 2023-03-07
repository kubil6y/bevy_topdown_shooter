use crate::prelude::*;
use bevy::prelude::*;
use camera::MyCameraPlugin;
use player::PlayerPlugin;

mod camera;
mod components;
mod constants;
mod enemy;
mod player;
mod prelude;
mod resources;
mod shared;
mod events;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Topdown Shooter".to_owned(),
                height: 600.,
                width: 400.,
                ..default()
            },
            ..default()
        }))
        .add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new().with_system(setup),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(PlayerPlugin)
        .add_plugin(MyCameraPlugin)
        .add_plugin(SharedPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
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
