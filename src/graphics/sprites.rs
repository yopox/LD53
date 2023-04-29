pub type X = usize;
pub type Y = usize;
pub type INDEX = usize;
pub type BG = u8;
pub type FG = u8;
pub type FLIP = bool;
pub type ROTATION = u8;
pub type TILE = (X, Y, INDEX, BG, FG, FLIP, ROTATION);

pub const CASH_KNIGHT: [TILE; 9] = [
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 939, 0, 2, false, 0),
    (2, 2, 0, 0, 1, false, 0),
    (0, 1, 868, 0, 2, false, 0),
    (1, 1, 228, 0, 1, false, 0),
    (2, 1, 941, 0, 2, false, 3),
    (0, 0, 343, 0, 1, true, 0),
    (1, 0, 206, 0, 1, false, 0),
    (2, 0, 343, 0, 1, false, 0),
];
