pub type X = usize;
pub type Y = usize;
pub type INDEX = usize;
pub type BG = u8;
pub type FG = u8;
pub type FLIP = bool;
pub type ROTATION = u8;
pub type TILE = (X, Y, INDEX, BG, FG, FLIP, ROTATION);

pub const DRONE_1: [TILE; 6] = [
    (0, 1, 222, 16, 15, false, 0),
    (1, 1, 644, 15, 12, false, 0),
    (2, 1, 222, 16, 15, false, 1),
    (0, 0, 153, 16, 3, true, 3),
    (1, 0, 0, 16, 3, false, 0),
    (2, 0, 153, 16, 3, false, 3),
];

pub const TOWER_1: [TILE; 3] = [
    (0, 2, 478, 16, 8, false, 0),
    (0, 1, 1022, 16, 3, false, 0),
    (0, 0, 451, 16, 3, false, 0),
];
