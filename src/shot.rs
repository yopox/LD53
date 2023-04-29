use std::borrow::Borrow;

use bevy::prelude::*;
use bevy_tweening::TweenCompleted;
use strum_macros::EnumIter;

use crate::collision::HitBox;
use crate::graphics::palette::{Palette, TRANSPARENT};
use crate::graphics::sprites::TILE;
use crate::util::tweening::SHOT_DESPAWNED;

#[derive(Component, Copy, Clone)]
pub struct Shot {
    damages: f32,
    speed: f32,
}

#[derive(Copy, Clone, EnumIter, Debug)]
pub enum Shots {
    Basic,
}

impl Shots {
    pub const fn get_default_damages(&self) -> f32 {
        match self {
            Self::Basic => 6.,
        }
    }

    const fn get_shot(&self) -> Shot {
        match &self {
            Self::Basic => Shot {
                damages: 6.,
                speed: 120.,
            }
        }
    }

    pub fn instantiate(&self) -> (Shot, HitBox) {
        let (index, _, _, bg, _, _, _) = self.get_tile();
        let hitbox = HitBox::for_tile(index, <u8 as Into<Palette>>::into(bg) == Palette::Transparent).unwrap();
        (self.get_shot(), hitbox)
    }

    pub const fn get_tile(&self) -> TILE {
        match &self {
            Shots::Basic =>
                (877, 1, 222, 16, 15, false, 0)
        }
    }

    pub fn get_tiles(&self) -> &[TILE] {
        &[
            (877, 1, 222, 16, 15, false, 0),
        ]
    }

    pub fn get_speed(&self) -> f32 {
        self.get_shot().speed
    }
}

pub fn remove_shots(
    mut tween_completed: EventReader<TweenCompleted>,
    mut commands: Commands,
) {
    for event in tween_completed.iter() {
        if event.user_data == SHOT_DESPAWNED {
            if let Some(entity_commands) = commands.get_entity(event.entity) {
                entity_commands.despawn_recursive();
            }
        }
    }
}