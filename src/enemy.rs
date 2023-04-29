use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::graphics::{MainBundle, sprite_from_tile};
use crate::graphics::sprites::{TILE, CASH_KNIGHT, DEFAULT_PALETTE};
use crate::util;

#[derive(Debug, Clone)]
pub struct EnemyStats {
    hp: f32,
    speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Enemies {
    Drone
}

#[derive(Component)]
pub struct Enemy {
    class: Enemies,
    stats: EnemyStats,
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
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match self {
            Self::Drone => &CASH_KNIGHT,
        }
    }
}
