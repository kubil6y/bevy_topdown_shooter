use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct PlayerLaserFireEvent(pub Vec2);

#[derive(Debug, Default)]
pub struct EnemyLaserFireEvent(pub Vec2);

#[derive(Default)]
pub struct PlayerTakeHitEvent;

#[derive(Debug)]
pub struct EnemyTakeHitEvent(pub Entity, pub Vec3);

#[derive(Debug)]
//pub struct ExplosionEvent(pub Vec2);
pub struct ExplosionEvent {
    pub position: Vec2,
    pub with_sound: bool,
}

#[derive(Default)]
pub struct WaveCompleteEvent;

#[derive(Default)]
pub struct PlayerDeathEvent;
