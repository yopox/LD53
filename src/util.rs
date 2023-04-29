pub mod size {
    /// Tile size from the tileset
    const TILE_SIZE: usize = 8;

    /// Screen size in tiles
    pub const WIDTH: usize = 32;
    pub const HEIGHT: usize = 18;

    /// Camera scale
    pub const SCALE: f32 = 5.;

    /// Returns world coordinates for a tile, for instance `2` -> `(2 * TILE_SIZE) as f32 `.
    pub const fn tile_to_f32(tile: usize) -> f32 { (tile * TILE_SIZE) as f32 }
}

pub mod z_pos {
    pub const BACKGROUND: f32 = 0.;
    pub const TITLE_TEXT: f32 = 8.5;
    pub const TRANSITION: f32 = 9.;
    pub const GUI: f32 = 12.;
}

pub mod transition {
    use crate::util::size::HEIGHT;

    pub const HALF_HEIGHT: usize = HEIGHT / 2 - 1;
    pub const SPEED: u64 = 800;
}

pub mod tweening {
    pub const TRANSITION_OVER: u64 = 1;
    pub const DELAY: u64 = 200;
}

pub mod misc {
    pub const ANIMATION_INTERVAL: usize = 80;
}