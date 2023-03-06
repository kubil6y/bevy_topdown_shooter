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
        //.add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new().with_system(setup),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(PlayerPlugin)
        .add_plugin(MyCameraPlugin)
        .add_system(movement_system)
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

    let game_textures = GameTextures {
        player: asset_server.load(SPRITE_PLAYER_SHIP),
    };
    commands.insert_resource(game_textures);
}
