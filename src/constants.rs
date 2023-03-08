use bevy::prelude::{Color, Vec2};

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const BASE_SPEED: f32 = 300.;
pub const MAX_ENEMY_COUNT: i32 = 20;
pub const EXPLOSION_LENGTH: usize = 16;
pub const UPGRADE_ENEMY_KILL_COUNT: i32 = 10;
pub const ENEMY_LASER_SPEED_MULTIPLIER: f32 = 1.5;

pub const ENEMY_BASE_VELOCITY: Vec2 = Vec2::new(0., -0.3);
pub const ENEMY_BASE_FIRE_RATE: f64 = 1. / 60.;

pub const SPRITE_SCALE: f32 = 1.;
pub const SPRITE_PLAYER_SHIP: &str = "ships/6.png";
pub const SIZE_PLAYER_SHIP: (f32, f32) = (44., 36.);
pub const SPRITE_LASER_PLAYER: &str = "shots/6.png";
pub const SIZE_LASER_PLAYER: (f32, f32) = (18., 21.);
pub const SPRITE_LASER_ENEMY: &str = "shots/2.png";
pub const SIZE_LASER_ENEMY: (f32, f32) = (8., 17.);
pub const SPRITE_SHEET_EXPLOSION: &str = "effects/explo_a_sheet.png";

pub const SPRITE_ENEMY_SHIP: &str = "ships/3.png";
pub const SIZE_ENEMY_SHIP: (f32, f32) = (46., 45.);

pub const AUDIO_PLAYER_SHOOT: &str = "audio/player_shoot.ogg";
pub const AUDIO_ENEMY_SHOOT: &str = "audio/enemy_shoot.ogg";
pub const AUDIO_HIT: &str = "audio/hit.ogg";
pub const AUDIO_EXPLOSION: &str = "audio/explosion.ogg";
pub const AUDIO_DEATH: &str = "audio/death.ogg";
pub const AUDIO_POWERUP: &str = "audio/powerup.ogg";
pub const AUDIO_GOLD: &str = "audio/gold.ogg";
