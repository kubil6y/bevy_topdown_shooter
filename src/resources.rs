use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerState {
    pub health: i32,
    pub golds: i32,
    pub score: i32,
    pub is_alive: bool,
    pub death_sound_played: bool,
}

impl PlayerState {
    pub fn decrement_health(&mut self) {
        if self.is_alive {
            self.health -= 1;
            if self.health <= 0 {
                self.die()
            }
        }
    }

    pub fn spawn(&mut self) {
        *self = Self {
            is_alive: true,
            ..Default::default()
        }
    }

    pub fn die(&mut self) {
        self.is_alive = false;
    }

    pub fn increment_gold(&mut self) {
        self.golds += 1;
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            is_alive: false,
            health: 3,
            golds: 0,
            score: 0,
            death_sound_played: false,
        }
    }
}

#[derive(Resource)]
pub struct EnemyCount(pub i32);

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub laser_player: Handle<Image>,
    pub laser_enemy: Handle<Image>,
    pub enemy: Handle<Image>,
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
