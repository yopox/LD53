use crate::graphics::palette::Palette;
use crate::graphics::tile::Rotation;

pub type X = usize;
pub type Y = usize;
pub type INDEX = usize;
pub type BG = usize;
pub type FG = usize;
pub type TILE = (X, Y, INDEX, BG, FG, Rotation);

pub const CASH_KNIGHT: [TILE; 9] = [
    (0, 2, 0, 0, 1, Rotation::No),
    (1, 2, 939, 0, 2, Rotation::No),
    (2, 2, 0, 0, 1, Rotation::No),
    (0, 1, 868, 0, 2, Rotation::No),
    (1, 1, 228, 0, 1, Rotation::No),
    (2, 1, 941, 0, 2, Rotation::Left),
    (0, 0, 343, 0, 1, Rotation::Flip),
    (1, 0, 206, 0, 1, Rotation::No),
    (2, 0, 343, 0, 1, Rotation::No),
];

pub const DEFAULT_PALETTE: [Palette; 3] = [
    Palette::Black,
    Palette::White,
    Palette::Transparent,
];
