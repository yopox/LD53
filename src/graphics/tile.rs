use bevy::prelude::*;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use crate::graphics::palette::Palette;
use crate::graphics::sprite;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Rotation {
    #[default]
    No = 0,
    Right,
    Flip,
    Left,
}

impl Into<u8> for Rotation {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub index: usize,
    pub bg: Palette,
    pub fg: Palette,
    pub flip: bool,
    pub rotation: Rotation,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            index: 0,
            bg: Palette::Transparent,
            fg: Palette::Transparent,
            flip: false,
            rotation: Rotation::default(),
        }
    }
}

impl Tile {
    pub fn new(index: usize, flip: bool, rotation: Rotation) -> Self {
        Tile { index, flip, rotation, ..Tile::default() }
    }

    pub fn from_index(index: usize) -> Self {
        Tile { index, ..Tile::default() }
    }

    pub fn with_fg(self, fg: Palette) -> Self {
        Tile { fg, ..self }
    }

    pub fn with_rotation(self, rotation: Rotation) -> Self {
        Tile { rotation, ..self }
    }

    pub fn flip(self) -> Self {
        Tile { flip: !self.flip, ..self }
    }

    pub fn sprite(&self, x: usize, y: usize, z: f32, atlas: &Handle<TextureAtlas>) -> TextModeSpriteSheetBundle {
        sprite(self.index, x, y, z, self.bg, self.fg, self.flip, self.rotation.into(), atlas.clone())
    }
}
