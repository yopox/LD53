use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use bevy_tweening::TweeningPlugin;

use crate::graphics::animation::AnimationPlugin;
use crate::graphics::grid::GridPlugin;
use crate::graphics::loading::LoadingPlugin;
use crate::graphics::palette::Palette;
use crate::graphics::sprites::TILE;
use crate::graphics::text::TextPlugin;
use crate::graphics::transition::TransitionPlugin;
use crate::util::size::tile_to_f32;

pub mod text;
pub mod loading;
pub mod palette;
pub mod transition;
mod animation;
pub mod tile;
pub mod sprites;
pub mod grid;

#[derive(Bundle, Debug, Default)]
pub struct MainBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: VisibilityBundle,
}

impl MainBundle {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        MainBundle {
            transform: Transform::from_xyz(x, y, z),
            ..default()
        }
    }

    pub fn from_tiles(x: usize, y: usize, z: f32) -> Self {
        Self::from_xyz(tile_to_f32(x), tile_to_f32(y), z)
    }
}

pub fn sprite(
    index: usize,
    x: usize, y: usize, z: f32,
    bg: Palette, fg: Palette,
    flip: bool, rotation: u8,
    atlas: Handle<TextureAtlas>,
) -> TextModeSpriteSheetBundle {
    TextModeSpriteSheetBundle {
        sprite: TextModeTextureAtlasSprite {
            bg: bg.into(),
            fg: fg.into(),
            alpha: 1.0,
            index,
            flip_x: flip,
            rotation,
            anchor: Anchor::BottomLeft,
            ..default()
        },
        texture_atlas: atlas,
        transform: Transform {
            translation: Vec3::new(tile_to_f32(x), tile_to_f32(y) , z),
            ..default()
        },
        ..default()
    }
}

pub fn sprite_from_tile (
    builder: &mut ChildBuilder,
    tiles: &[TILE],
    atlas: &Handle<TextureAtlas>,
    z: f32,
) {
    for &(x, y, i, bg, fg, flip, rotation) in tiles {
        builder.spawn(
            sprite(
                i, x, y, z,
                bg.into(),
                fg.into(),
                flip,
                rotation,
                atlas.clone(),
            )
        );
    }
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TextPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(TransitionPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(TweeningPlugin)
            .add_plugin(GridPlugin)
        ;
    }
}