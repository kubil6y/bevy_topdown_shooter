use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
}

#[derive(Resource)]
pub struct GameState {
    pub is_debug: bool,
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
