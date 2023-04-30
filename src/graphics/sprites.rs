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
    Carrier,
    Super,
}

impl DroneModels {
    pub fn get_tiles(&self) -> &'static [TILE] {
        match self {
            DroneModels::Simple => &DRONE_1,
            DroneModels::Carrier => &DRONE_2,
            DroneModels::Super => &DRONE_3,
        }
    }

    /// Returns the offset for the package in pixels from the bottom-left tile.
    pub fn package_offset(&self) -> Vec2 {
        match self {
            DroneModels::Simple => Vec2::new(0., -1.),
            DroneModels::Carrier => Vec2::new(4., 7.),
            DroneModels::Super => Vec2::new(8., -1.),
        }
    }

    pub fn get_size(&self) -> Vec2 {
        match self {
            DroneModels::Simple => Vec2::new(tile_to_f32(1), tile_to_f32(2)),
            DroneModels::Carrier => Vec2::new(tile_to_f32(2), tile_to_f32(2)),
            DroneModels::Super => Vec2::new(tile_to_f32(3), tile_to_f32(2)),
        }
    }
}

const DRONE_1: &'static [TILE] = &[
    (0, 1, 1, 16, 9, false, 0),
    (0, 0, 1, 16, 15, false, 0),
];

const DRONE_2: &'static [TILE] = &[
    (0, 1, 10, 16, 9, false, 0),
    (1, 1, 11, 16, 9, false, 0),
    (0, 0, 42, 16, 9, false, 0),
    (1, 0, 43, 16, 9, false, 0),
];

const DRONE_3: &'static [TILE] = &[
    (0, 1, 12, 16, 9, false, 0),
    (1, 1, 13, 10, 9, false, 0),
    (2, 1, 14, 16, 9, false, 0),
    (0, 0, 44, 16, 6, false, 0),
    (1, 0, 45, 16, 6, false, 0),
    (2, 0, 46, 16, 6, false, 0),
];

pub const TOWER_1: &'static [TILE] = &[
    (0, 2, 64, 16, 8, false, 0),
    (1, 2, 64, 16, 8, false, 0),
    (0, 1, 96, 16, 10, false, 0),
    (1, 1, 96, 16, 10, false, 0),
    (0, 0, 96, 16, 10, false, 0),
    (1, 0, 96, 16, 10, false, 0),
];

pub const TOWER_2: &'static [TILE] = &[
    (0, 2, 64, 16, 7, false, 0),
    (1, 2, 64, 16, 7, false, 0),
    (0, 1, 96, 16, 9, false, 0),
    (1, 1, 96, 16, 9, false, 0),
    (0, 0, 96, 16, 9, false, 0),
    (1, 0, 96, 16, 9, false, 0),
];

pub const TOWER_3: &'static [TILE] = &[
    (0, 2, 64, 16, 6, false, 0),
    (1, 2, 64, 16, 6, false, 0),
    (0, 1, 96, 16, 8, false, 0),
    (1, 1, 96, 16, 8, false, 0),
    (0, 0, 96, 16, 8, false, 0),
    (1, 0, 96, 16, 8, false, 0),
];
