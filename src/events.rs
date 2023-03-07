use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct PlayerLaserFireEvent(pub Vec2);

#[derive(Debug, Default)]
pub struct EnemyLaserFireEvent(pub Vec2);

#[derive(Default)]
pub struct PlayerTakeHitEvent;
