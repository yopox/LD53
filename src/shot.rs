use std::borrow::Borrow;

use bevy::prelude::*;
use bevy_tweening::TweenCompleted;
use strum_macros::EnumIter;

use crate::collision::{body_size, BodyType, HitBox};
use crate::graphics::sprites::TILE;
use crate::util::tweening::SHOT_DESPAWNED;

#[derive(Component, Copy, Clone)]
pub struct Shot {
    pub damages: f32,
    pub speed: f32,
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
        let body_size = body_size(self.get_tiles());
        let solid_body = HitBox {
            body_type: BodyType::ShipShot,
            width: body_size.x,
            height: body_size.y,
            bottom_right_anchor: false,
        };
        (self.get_shot(), solid_body)
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