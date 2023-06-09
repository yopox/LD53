use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::collision::body_size;
use crate::drones::Drones;
use crate::tower::{Tower, Towers};
use crate::util::size::{f32_tile_to_f32, tile_to_f32};

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

        pub const BOMB_RANGE: f32 = 3.5 * TILE_SIZE as f32;
        pub const OMEGA_RANGE: f32 = 6. * TILE_SIZE as f32;
    }
}

pub mod z_pos {
    // Background
    pub const TITLE_TEXT: f32 = 1.;
    pub const ROAD: f32 = 3.;

    pub const TOWER_RADIUS: f32 = 3.25;
    pub const CURSOR: f32 = 3.5;

    // Battle elements
    pub const BATTLE_MIN: f32 = 6.;
    pub const BATTLE_MAX: f32 = 6.99;
    pub const SHOT: f32 = 7.;
    pub const BOMB: f32 = 7.1;
    pub const EXPLOSION: f32 = 8.;
    pub const TRANSPARENT_TOWER: f32 = 9.;

    pub const ATTACHED_PACKAGE_OFFSET: f32 = -1. / 4096.;

    // GUI
    pub const GUI_BG: f32 = 11.;
    pub const GUI_FG: f32 = 12.;
    pub const POPUP_BG: f32 = 13.;
    pub const POPUP_FG: f32 = 14.;
    pub const TRANSITION: f32 = 80.;
}

pub mod transition {
    use crate::util::size::HEIGHT;

    pub const HALF_HEIGHT: usize = HEIGHT / 2;
    pub const SPEED: u64 = 800;
}

pub mod tweening {
    // UID of events
    pub const TRANSITION_OVER: u64 = 1;
    pub const SHOT_DESPAWN: u64 = 2;
    pub const BOMB_EXPLODED: u64 = 3;
    pub const DRONE_DESPAWN: u64 = 4;

    // durations of tweenings
    pub const DELAY: u64 = 200;
    pub const DRONE_DEATH_FREEZE: u64 = 400;
    pub const DRONE_DEATH_ALPHA: u64 = 800;
    pub const DRONE_DEATH_POS: u64 = 1200;
    pub const PACKAGE_DROP: u64 = 800;
}

pub mod package {
    pub const MONEY_SELL: u16 = 10;
    pub const MONEY_SMALL: u16 = 20;
    pub const MONEY_BIG: u16 = 60;
    pub const MONEY_CURSE: u16 = 30;
}

pub mod misc {
    pub const ANIMATION_INTERVAL: usize = 80;

    pub const SLOW_DOWN_DELAY: f32 = 10.;
}

pub const fn with_z(Vec3 { x, y, .. }: Vec3, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn battle_z_from_y(y: f32) -> f32 {
    use crate::util::size::HEIGHT;
    use crate::util::z_pos::{BATTLE_MAX, BATTLE_MIN};

    let max_y = tile_to_f32(HEIGHT + 10);

    BATTLE_MIN + (BATTLE_MAX - BATTLE_MIN) / max_y * (max_y - y)
}

pub fn vec3_with_battle_z(x: f32, y: f32) -> Vec3 {
    vec3(x, y, battle_z_from_y(y))
}

pub fn vec2_with_battle_z(Vec2 { x, y }: Vec2) -> Vec3 {
    vec3(x, y, battle_z_from_y(y))
}

pub fn cursor_pos(
    windows: Query<&Window>,
) -> Option<Vec2> {
    let window = windows.get_single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else { return None; };
    return Some(Vec2::new(cursor_pos.x / size::SCALE, cursor_pos.y / size::SCALE));
}

pub fn grid_to_tower_pos(x: usize, y: usize, t: Towers) -> Vec2 {
    let size = body_size(t.get_tiles());
    let dx = (tile_to_f32(2) - size.x) / 2.;
    let x = tile_to_f32(2 * x) + dx;
    let y = tile_to_f32(2 * y + size::GUI_HEIGHT) + f32_tile_to_f32(0.5);
    return Vec2::new(x, y);
}

/// Returns true if [p] is in the rectangle with [o] bottom-left origin and [size] dimensions.
pub fn is_in(p: Vec2, o: Vec2, size: Vec2) -> bool {
    p.x >= o.x && p.x <= o.x + size.x && p.y >= o.y && p.y <= o.y + size.y
}

pub fn tower_center(x: usize, y: usize) -> Vec2 {
    vec2(tile_to_f32(2 * x + 1), tile_to_f32(2 * y + 1 + size::GUI_HEIGHT))
}

pub fn tower_to_enemy_distance(tower: &Tower, enemy_pos: Vec2, enemy: Drones) -> f32 {
    let enemy_size = body_size(enemy.get_tiles());
    let enemy_center = enemy_pos + enemy_size / 2.;
    let tower_center = tower_center(tower.x, tower.y);
    tower_center.distance(enemy_center)
}