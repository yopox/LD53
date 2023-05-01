use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use bevy_tweening::{component_animator_system, TweeningPlugin};

use crate::graphics::animation::AnimationPlugin;
use crate::graphics::grid::GridPlugin;
use crate::graphics::gui::GuiPlugin;
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
pub mod animation;
pub mod tile;
pub mod sprites;
pub mod grid;
pub mod package;
pub mod gui;
pub mod tween;
pub mod circle;

#[derive(Bundle, Debug, Default)]
pub struct MainBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: VisibilityBundle,
}

impl MainBundle {
    pub fn from_transform(transform: Transform) -> Self {
        MainBundle { transform, ..default() }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        MainBundle::from_transform(Transform::from_translation(translation))
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        MainBundle::from_transform(Transform::from_xyz(x, y, z))
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
    sprite_f32(
        index,
        tile_to_f32(x), tile_to_f32(y), z,
        bg, fg, flip, rotation, atlas.clone(),
    )
}

pub fn sprite_f32(
    index: usize,
    x: f32, y: f32, z: f32,
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
            translation: Vec3::new(x, y, z),
            ..default()
        },
        ..default()
    }
}

pub fn sprite_from_tile(
    builder: &mut ChildBuilder,
    tiles: &[TILE],
    atlas: &Handle<TextureAtlas>,
    z: f32,
) {
    sprite_from_tile_with_alpha(builder, tiles, atlas, z, 1.0);
}

pub fn sprite_from_tile_with_alpha(
    builder: &mut ChildBuilder,
    tiles: &[TILE],
    atlas: &Handle<TextureAtlas>,
    z: f32,
    alpha: f32,
) {
    sprite_from_tile_with_alpha_and_x_offset(builder, tiles, atlas, z, alpha, 0.0);
}

pub fn sprite_from_tile_with_alpha_and_x_offset(
    builder: &mut ChildBuilder,
    tiles: &[TILE],
    atlas: &Handle<TextureAtlas>,
    z: f32,
    alpha: f32,
    x_offset: f32,
) {
    for &(x, y, i, bg, fg, flip, rotation) in tiles {
        let mut bundle = sprite_f32(
            i, tile_to_f32(x) + x_offset, tile_to_f32(y), z,
            bg.into(), fg.into(),
            flip, rotation,
            atlas.clone(),
        );
        bundle.sprite.alpha = alpha;
        builder.spawn(bundle);
    }
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(component_animator_system::<TextModeTextureAtlasSprite>)
            .add_plugin(TextPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(TransitionPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(TweeningPlugin)
            .add_plugin(GridPlugin)
            .add_plugin(GuiPlugin)
            .add_startup_system(circle::setup)
        ;
    }
}