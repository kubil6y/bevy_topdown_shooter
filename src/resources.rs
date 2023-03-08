use bevy::prelude::*;

use crate::prelude::{BASE_SPEED, ENEMY_BASE_FIRE_RATE, ENEMY_BASE_VELOCITY};

#[derive(Resource)]
pub struct BackgroundSpeed(pub f32);

#[derive(Resource, Debug)]
pub struct PlayerState {
    pub health: i32,
    pub golds: i32,
    pub score: i32,
    pub is_alive: bool,
    pub death_sound_played: bool,
    pub speed: f32,
}

impl PlayerState {
    pub fn increment_health(&mut self) {
        if self.is_alive && self.health < 3 {
            self.health += 1;
        }
    }

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

    pub fn upgrade(&mut self) {
        self.speed += 0.10;
        self.increment_health();
        self.increment_gold();
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
            speed: BASE_SPEED,
        }
    }
}

#[derive(Resource)]
pub struct EnemyCount(pub i32);

#[derive(Resource)]
pub struct EnemyAttributes {
    pub fire_rate: f64,
    pub velocity: Vec2,
}

impl EnemyAttributes {
    pub fn reset(&mut self) {
        *self = Default::default();
    }
    pub fn upgrade(&mut self) {
        self.velocity = Vec2::new(self.velocity.x, self.velocity.y - 0.05);
    }
}

impl Default for EnemyAttributes {
    fn default() -> Self {
        Self {
            fire_rate: ENEMY_BASE_FIRE_RATE,
            velocity: ENEMY_BASE_VELOCITY,
        }
    }
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub laser_player: Handle<Image>,
    pub laser_enemy: Handle<Image>,
    pub enemy: Handle<Image>,
    pub background: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct FontAssets {
    pub ui: Handle<Font>,
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
    pub theme_song: Handle<AudioSource>,
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
