use crate::tower::{Tower, Towers};
use crate::util::size::f32_tile_to_f32;

/// Time between two shots in seconds
pub fn reload_delay(tower: &Tower) -> f32 {
    match (tower.model, tower.rank) {
        (Towers::Lightning, 1) => 4.,
        (Towers::Lightning, 2) => 3.,
        (Towers::Lightning, _) => 2.,

        (Towers::PaintBomb, 1) => 6.,
        (Towers::PaintBomb, 2) => 5.,
        (Towers::PaintBomb, _) => 4.,

        (Towers::Scrambler, 1) => 5.,
        (Towers::Scrambler, 2) => 5.,
        (Towers::Scrambler, _) => 5.,
    }
}

/// Radius of the circular range in zoomed pixels
pub fn range(tower: &Tower) -> f32 {
    f32_tile_to_f32(match (tower.model, tower.rank) {
        (Towers::Lightning, 1) => 5.0,
        (Towers::Lightning, 2) => 5.5,
        (Towers::Lightning, _) => 6.0,

        (Towers::PaintBomb, 1) => 8.0,
        (Towers::PaintBomb, 2) => 9.0,
        (Towers::PaintBomb, _) => 10.,

        (Towers::Scrambler, 1) => 3.0,
        (Towers::Scrambler, 2) => 3.5,
        (Towers::Scrambler, _) => 4.0,
    })
}

/// Damage of this tower's shots
pub fn damage(tower: &Tower) -> f32 {
    match (tower.model, tower.rank) {
        (Towers::Lightning, 1) => 3.0,
        (Towers::Lightning, 2) => 5.0,
        (Towers::Lightning, _) => 10.0,

        (Towers::PaintBomb, 1) => 8.0,
        (Towers::PaintBomb, 2) => 10.0,
        (Towers::PaintBomb, _) => 12.0,

        _ => 0.,
    }
}

/// Speed of this tower's shots
pub fn shot_speed(tower: &Tower) -> f32 {
    match (tower.model, tower.rank) {
        _ => 120.,
    }
}