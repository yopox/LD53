use bevy::prelude::Vec3;

pub mod size {
    /// Tile size from the tileset
    const TILE_SIZE: usize = 8;

    /// Screen size in tiles
    pub const WIDTH: usize = 20;
    pub const HEIGHT: usize = 12;

    /// Camera scale
    pub const SCALE: f32 = 8.;

    /// Returns world coordinates for a tile, for instance `2` -> `(2 * TILE_SIZE) as f32 `.
    pub const fn tile_to_f32(tile: usize) -> f32 { (tile * TILE_SIZE) as f32 }

    pub fn path_to_f32(pos: f32) -> f32 { pos * TILE_SIZE as f32 }
}

pub mod z_pos {
    pub const BACKGROUND: f32 = 0.;
    pub const TITLE_TEXT: f32 = 1.;
    pub const GRID: f32 = 2.;
    pub const ROAD: f32 = 2.5;
    pub const PACKAGES: f32 = 5.99;
    pub const ENEMIES: f32 = 6.;
    pub const SHOT: f32 = 7.;
    pub const TRANSITION: f32 = 10.;
    pub const GUI: f32 = 11.;
}

pub mod transition {
    use crate::util::size::HEIGHT;

    pub const HALF_HEIGHT: usize = HEIGHT / 2 - 1;
    pub const SPEED: u64 = 800;
}

pub mod tweening {
    pub const TRANSITION_OVER: u64 = 1;
    pub const SHOT_DESPAWNED: u64 = 2;
    pub const DELAY: u64 = 200;
}

pub mod misc {
    pub const ANIMATION_INTERVAL: usize = 80;
}

pub const fn with_z(Vec3 { x, y, .. }: Vec3, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}