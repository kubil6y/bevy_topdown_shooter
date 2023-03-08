use bevy::{
    prelude::{Component, Vec2},
    time::{Timer, TimerMode},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Explosion(pub Timer);

impl Default for Explosion {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

#[derive(Component, Default)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Collision(pub Vec2);

impl From<(f32, f32)> for Collision {
    fn from(value: (f32, f32)) -> Self {
        Self(Vec2::new(value.0, value.1))
    }
}

#[derive(Component, Default)]
pub struct UIFont;
