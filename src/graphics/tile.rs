use bevy::prelude::*;
use bevy_text_mode::TextModeSpriteSheetBundle;

use crate::graphics::palette::Palette;
use crate::graphics::sprite;

#[derive(Copy, Clone)]
pub struct Tile {
    pub index: usize,
    pub bg: Palette,
    pub fg: Palette,
    pub flip: bool,
    pub rotation: u8,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            index: 0,
            bg: Palette::Transparent,
            fg: Palette::Transparent,
            flip: false,
            rotation: 0,
        }
    }
}

#[allow(dead_code)]
impl Tile {
    pub fn new(index: usize, flip: bool, rotation: u8) -> Self {
        Tile { index, flip, rotation, ..Tile::default() }
    }

    pub fn from_index(index: usize) -> Self {
        Tile { index, ..Tile::default() }
    }

    pub fn with_fg(self, fg: Palette) -> Self {
        Tile { fg, ..self }
    }

    pub fn with_rotation(self, rotation: u8) -> Self {
        Tile { rotation, ..self }
    }

    pub fn flip(self) -> Self {
        Tile { flip: !self.flip, ..self }
    }

    pub fn sprite(&self, x: usize, y: usize, z: f32, atlas: &Handle<TextureAtlas>) -> TextModeSpriteSheetBundle {
        sprite(self.index, x, y, z, self.bg, self.fg, self.flip, self.rotation, atlas.clone())
    }
}
