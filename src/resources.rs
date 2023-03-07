use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub laser_player: Handle<Image>,
    pub laser_enemy: Handle<Image>,
}

#[derive(Resource)]
pub struct AudioAssets {
    pub player_shoot: Handle<AudioSource>,
    pub enemy_shoot: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
    pub powerup: Handle<AudioSource>,
    pub gold: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl WindowSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

// TODO: remove this
#[derive(Resource)]
pub struct GameState {
    pub is_debug: bool,
}

impl GameState {
    //pub fn set_debug(&mut self, is_debug: bool) {
    //self.is_debug = is_debug;
    //}
}

impl Default for GameState {
    fn default() -> Self {
        Self { is_debug: true }
    }
}
