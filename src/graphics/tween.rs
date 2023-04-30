use std::time::Duration;

use bevy_text_mode::TextModeTextureAtlasSprite;
use bevy_tweening::{EaseFunction, Lens, Tween};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformTextModeSpriteAlphaLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<TextModeTextureAtlasSprite> for TransformTextModeSpriteAlphaLens {
    fn lerp(&mut self, target: &mut TextModeTextureAtlasSprite, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.alpha = value;
    }
}

pub fn tween_text_mode_sprite_opacity(ms: u64, appear: bool) -> Tween<TextModeTextureAtlasSprite> {
    Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(ms),
        TransformTextModeSpriteAlphaLens {
            start: if appear { 0. } else { 1. },
            end: if appear { 1. } else { 0. },
        },
    )
}