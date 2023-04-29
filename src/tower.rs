use bevy::prelude::*;
use crate::graphics::{MainBundle, sprite_from_tile};
use crate::graphics::sprites::{CASH_KNIGHT, DEFAULT_PALETTE, TILE};
use crate::shot::{Shot, Shots};
use crate::util;

#[derive(Component)]
pub struct Tower {
    class: Towers,
    /// Time between two shots in seconds
    reloading_delay: f32,
    shot: Shots,
}

#[derive(Debug, Copy, Clone)]
pub enum Towers {
    Basic,
}

impl Towers {
    pub const fn instantiate(&self) -> Tower {
        match &self {
            Towers::Basic => Tower {
                class: *self,
                reloading_delay: 10.,
                shot: Shots::Basic,
            }
        }
    }

    pub const fn get_tiles(&self) -> &[TILE] {
        match &self {
            Towers::Basic => &CASH_KNIGHT,
        }
    }
}