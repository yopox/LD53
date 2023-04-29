use bevy::math::Vec2;

use util::size::tile_to_f32;

use crate::util;

pub type X = usize;
pub type Y = usize;
pub type INDEX = usize;
pub type BG = u8;
pub type FG = u8;
pub type FLIP = bool;
pub type ROTATION = u8;
pub type TILE = (X, Y, INDEX, BG, FG, FLIP, ROTATION);

pub enum DroneModels {
    Simple,
    Super,
}

impl DroneModels {
    pub fn get_tiles(&self) -> &'static [TILE] {
        match self {
            DroneModels::Simple => &DRONE_1,
            DroneModels::Super => &DRONE_2,
        }
    }

    /// Returns the offset for the package in pixels from the bottom-left tile.
    pub fn package_offset(&self) -> Vec2 {
        match self {
            DroneModels::Simple => Vec2::new(0., -1.),
            DroneModels::Super => Vec2::new(8., -3.),
        }
    }

    pub fn get_size(&self) -> Vec2 {
        match self {
            DroneModels::Simple => Vec2::new(tile_to_f32(1), tile_to_f32(2)),
            DroneModels::Super => Vec2::new(tile_to_f32(3), tile_to_f32(2)),
        }
    }
}

const DRONE_1: &'static [TILE] = &[
    (0, 1, 338, 16, 9, false, 0),
    (0, 0, 458, 16, 15, true, 2),
];

const DRONE_2: &'static [TILE] = &[
    (0, 1, 222, 16, 9, false, 0),
    (1, 1, 644, 9, 12, false, 0),
    (2, 1, 222, 16, 9, false, 1),
    (0, 0, 153, 16, 10, true, 3),
    (1, 0, 0, 16, 10, false, 0),
    (2, 0, 153, 16, 10, false, 3),
];

pub const TOWER_1: &'static [TILE] = &[
    (0, 1, 478, 16, 8, false, 0),
    (0, 0, 451, 16, 1, false, 0),
];
