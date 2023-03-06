use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Default)]
pub struct Movable {
    pub auto_despawn: bool,
    pub speed: Vec2,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct FromEnemy;
