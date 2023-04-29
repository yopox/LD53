use bevy::prelude::*;

use crate::graphics::sprites;
use crate::graphics::sprites::TILE;

#[derive(Debug, Clone)]
pub struct EnemyStats {
    pub(crate) hp: f32,
    speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Enemies {
    Drone
}

#[derive(Component)]
pub struct Enemy {
    class: Enemies,
    pub(crate) stats: EnemyStats,
    pub(crate) advance: f32,
}

impl Enemies {
    pub const fn get_default_stats(&self) -> EnemyStats {
        match self {
            Self::Drone => EnemyStats {
                hp: 10.,
                speed: 32.,
            }
        }
    }

    pub fn instantiate(&self) -> Enemy {
        Enemy {
            class: *self,
            stats: self.get_default_stats().clone(),
            advance: 0.,
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match self {
            Self::Drone => &sprites::DRONE_1,
        }
    }
}
