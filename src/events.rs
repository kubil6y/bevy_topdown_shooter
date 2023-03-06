use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug)]
pub struct PlayerLaserFireEvent(pub Vec2);
#[derive(Debug)]
pub struct EnemyLaserFireEvent(pub Vec2);
