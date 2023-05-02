use bevy::math::Vec2;
use strum_macros::EnumIter;

use crate::collision::body_size;

pub type X = usize;
pub type Y = usize;
pub type INDEX = usize;
pub type BG = u8;
pub type FG = u8;
pub type FLIP = bool;
pub type ROTATION = u8;
pub type TILE = (X, Y, INDEX, BG, FG, FLIP, ROTATION);

#[derive(EnumIter)]
pub enum DroneModels {
    Simple1,
    Simple2,
    Simple3,
    Medium1,
    Medium2,
    Medium3,
    Medium4,
    Big1,
    Big2,
    Invader,
}

impl DroneModels {
    pub fn get_tiles(&self) -> &'static [TILE] {
        match self {
            DroneModels::Simple1 => &DRONE_SMALL_1,
            DroneModels::Simple2 => &DRONE_SMALL_2,
            DroneModels::Simple3 => &DRONE_SMALL_3,
            DroneModels::Medium1 => &DRONE_MEDIUM_1,
            DroneModels::Medium2 => &DRONE_MEDIUM_2,
            DroneModels::Medium3 => &DRONE_MEDIUM_3,
            DroneModels::Medium4 => &DRONE_MEDIUM_4,
            DroneModels::Big1 => &DRONE_BIG_1,
            DroneModels::Big2 => &DRONE_BIG_2,
            DroneModels::Invader => &INVADER,
        }
    }

    /// Returns the offset for the package in pixels from the bottom-left tile.
    pub fn package_offset(&self) -> Vec2 {
        match self {
            DroneModels::Simple1 => Vec2::new(0., -1.),
            DroneModels::Simple2 => Vec2::new(0., -1.),
            DroneModels::Simple3 => Vec2::new(0., -1.),
            DroneModels::Medium1 => Vec2::new(4., 7.),
            DroneModels::Medium2 => Vec2::new(4., -3.),
            DroneModels::Medium3 => Vec2::new(4., -2.),
            DroneModels::Medium4 => Vec2::new(4., -5.),
            DroneModels::Big1 => Vec2::new(8., -4.),
            DroneModels::Big2 => Vec2::new(8., -1.),
            DroneModels::Invader => Vec2::new(8., 2.),
        }
    }

    pub fn get_hitbox(&self) -> Vec2 {
        match self {
            DroneModels::Simple1 | DroneModels::Simple2 | DroneModels::Simple3 =>
                Vec2::new(8., 13.),
            DroneModels::Medium1 =>
                Vec2::new(14., 11.),
            DroneModels::Medium2 | DroneModels::Medium3 | DroneModels::Medium4 =>
                Vec2::new(16., 8.),
            DroneModels::Big1 | DroneModels::Big2 =>
                Vec2::new(24., 8.),
            _ => body_size(self.get_tiles()),
        }
    }

    pub fn get_offset(&self) -> Vec2 {
        match self {
            DroneModels::Simple1 | DroneModels::Simple2 | DroneModels::Simple3 =>
                Vec2::new(0., 5.),
            DroneModels::Medium1 =>
                Vec2::new(1., 5.),
            DroneModels::Medium2 | DroneModels::Medium3 | DroneModels::Medium4 =>
                Vec2::new(0., 8.),
            DroneModels::Big1 | DroneModels::Big2 =>
                Vec2::new(0., 8.),
            _ => Vec2::ZERO,
        }
    }
}

const DRONE_SMALL_1: &'static [TILE] = &[
    (0, 2, 17, 16, 9, false, 0),
    (0, 1, 19, 16, 9, false, 0),
    (0, 0, 46, 16, 9, false, 0),
];

const DRONE_SMALL_2: &'static [TILE] = &[
    (0, 2, 17, 16, 9, false, 0),
    (0, 1, 50, 16, 9, false, 0),
    (0, 0, 46, 16, 9, false, 0),
];

const DRONE_SMALL_3: &'static [TILE] = &[
    (0, 2, 17, 16, 9, false, 0),
    (0, 1, 52, 16, 9, false, 0),
    (0, 0, 46, 16, 9, false, 0),
];

const DRONE_MEDIUM_1: &'static [TILE] = &[
    (0, 2, 9, 16, 9, false, 0),
    (1, 2, 9, 16, 9, true, 0),
    (0, 1, 44, 16, 9, false, 0),
    (1, 1, 44, 16, 9, true, 0),
    (0, 0, 42, 16, 9, false, 0),
    (1, 0, 42, 16, 9, true, 0),
];

const DRONE_MEDIUM_2: &'static [TILE] = &[
    (0, 2, 9, 16, 9, false, 0),
    (1, 2, 9, 16, 9, true, 0),
    (0, 1, 40, 16, 9, false, 0),
    (1, 1, 40, 16, 9, true, 0),
    (0, 0, 38, 16, 9, false, 0),
    (1, 0, 38, 16, 9, true, 0),
];

const DRONE_MEDIUM_3: &'static [TILE] = &[
    (0, 2, 9, 16, 9, false, 0),
    (1, 2, 9, 16, 9, true, 0),
    (0, 1, 40, 16, 9, true, 0),
    (1, 1, 40, 16, 9, false, 0),
    (0, 0, 36, 16, 9, false, 0),
    (1, 0, 36, 16, 9, true, 0),
];

const DRONE_MEDIUM_4: &'static [TILE] = &[
    (0, 2, 9, 16, 9, false, 0),
    (1, 2, 9, 16, 9, true, 0),
    (0, 1, 5, 16, 9, false, 0),
    (1, 1, 5, 16, 9, true, 0),
    (0, 0, 13, 16, 9, true, 2),
    (1, 0, 13, 16, 9, false, 2),
];

const DRONE_BIG_1: &'static [TILE] = &[
    (0, 2, 21, 16, 9, false, 0),
    (1, 2, 22, 16, 9, false, 0),
    (2, 2, 21, 16, 9, true, 0),
    (0, 1, 53, 16, 9, false, 0),
    (1, 1, 54, 16, 9, true, 0),
    (2, 1, 53, 16, 9, true, 0),
    (0, 0, 85, 16, 9, false, 0),
    (1, 0, 86, 16, 9, true, 0),
    (2, 0, 85, 16, 9, true, 0),
];

const DRONE_BIG_2: &'static [TILE] = &[
    (0, 2, 21, 16, 9, false, 0),
    (1, 2, 22, 16, 9, false, 0),
    (2, 2, 21, 16, 9, true, 0),
    (0, 1, 149, 16, 9, false, 0),
    (1, 1, 150, 16, 9, false, 0),
    (2, 1, 149, 16, 9, true, 0),
    (0, 0, 181, 16, 9, false, 0),
    (1, 0, 182, 16, 9, false, 0),
    (2, 0, 181, 16, 9, true, 0),
];

const INVADER: &'static [TILE] = &[
    (0, 2, 147, 16, 9, false, 0),
    (1, 2, 148, 16, 9, false, 0),
    (2, 2, 147, 16, 9, true, 0),
    (0, 1, 179, 16, 9, false, 0),
    (1, 1, 180, 16, 9, false, 0),
    (2, 1, 179, 16, 9, true, 0),
    (0, 0, 416, 16, 16, false, 0),
    (1, 0, 416, 16, 16, false, 0),
    (2, 0, 416, 16, 16, false, 0),
];

pub const TOWER_1: &'static [TILE] = &[
    (0, 2, 163, 16, 8, false, 0),
    (1, 2, 164, 16, 8, false, 0),
    (0, 1, 195, 16, 15, false, 0),
    (1, 1, 196, 16, 15, false, 0),
    (0, 0, 227, 16, 15, false, 0),
    (1, 0, 228, 16, 15, false, 0),
];

pub const TOWER_2: &'static [TILE] = &[
    (0, 2, 160, 16, 15, false, 0),
    (1, 2, 161, 16, 15, false, 0),
    (0, 1, 192, 16, 14, false, 0),
    (1, 1, 193, 16, 14, false, 0),
    (0, 0, 224, 16, 14, false, 0),
    (1, 0, 225, 16, 14, false, 0),
];

pub const TOWER_3: &'static [TILE] = &[
    (0, 2, 238, 16, 2, false, 0),
    (0, 1, 200, 16, 15, false, 0),
    (0, 0, 232, 16, 15, false, 0),
];
