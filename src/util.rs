use bevy::math::Vec2;
use bevy::prelude::{Query, Vec3, Window};

pub mod size {
    /// Tile size from the tileset
    const TILE_SIZE: usize = 8;

    /// Screen size in tiles
    pub const WIDTH: usize = 40;
    pub const HEIGHT: usize = 24;

    /// Camera scale
    pub const SCALE: f32 = 4.;

    pub const GUI_HEIGHT: usize = 6;
    pub const GRID_HEIGHT: usize = HEIGHT - GUI_HEIGHT;

    /// Returns world coordinates for a tile, for instance `2` -> `(2 * TILE_SIZE) as f32 `.
    pub const fn tile_to_f32(tile: usize) -> f32 { (tile * TILE_SIZE) as f32 }

    pub fn f32_tile_to_f32(pos: f32) -> f32 { pos * TILE_SIZE as f32 }

    pub fn is_oob(x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x >= WIDTH as isize || y >= GRID_HEIGHT as isize
    }

    pub mod battle {
        use crate::util::size::TILE_SIZE;

        pub const BOMB_RANGE: f32 = 2. * TILE_SIZE as f32;
    }
}

pub mod z_pos {
    // Background
    pub const BACKGROUND: f32 = 0.;
    pub const TITLE_TEXT: f32 = 1.;
    pub const GRID: f32 = 2.;
    pub const ROAD: f32 = 3.;

    pub const CURSOR: f32 = 3.5;

    // Battle elements
    pub const TOWERS: f32 = 4.;
    pub const PACKAGES: f32 = 5.99;
    pub const ENEMIES: f32 = 6.;
    pub const SHOT: f32 = 7.;
    pub const BOMB: f32 = 7.1;
    pub const EXPLOSION: f32 = 9.;

    // GUI
    pub const TRANSITION: f32 = 10.;
    pub const GUI_BG: f32 = 11.;
    pub const GUI_FG: f32 = 12.;
    pub const POPUP_BG: f32 = 13.;
    pub const POPUP_FG: f32 = 14.;
}

pub mod transition {
    use crate::util::size::HEIGHT;

    pub const HALF_HEIGHT: usize = HEIGHT / 2 - 1;
    pub const SPEED: u64 = 800;
}

pub mod tweening {
    pub const TRANSITION_OVER: u64 = 1;
    pub const SHOT_DESPAWN: u64 = 2;
    pub const BOMB_EXPLODED: u64 = 3;
    pub const DRONE_DESPAWN: u64 = 4;

    pub const DELAY: u64 = 200;
    pub const DRONE_DEATH_FREEZE: u64 = 400;
    pub const DRONE_DEATH_ALPHA: u64 = 800;
    pub const DRONE_DEATH_POS: u64 = 1200;
}

pub mod misc {
    pub const ANIMATION_INTERVAL: usize = 80;
}

pub const fn with_z(Vec3 { x, y, .. }: Vec3, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn cursor_pos(
    windows: Query<&Window>,
) -> Option<Vec2> {
    let window = windows.get_single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else { return None; };
    return Some(Vec2::new(cursor_pos.x / size::SCALE, cursor_pos.y / size::SCALE));
}