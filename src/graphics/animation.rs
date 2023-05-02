use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(wiggle);
    }
}

fn flip(sprite: &TextModeTextureAtlasSprite) -> TextModeTextureAtlasSprite {
    let mut new_sprite = sprite.clone();
    new_sprite.flip_x = !new_sprite.flip_x;
    return new_sprite;
}

fn rotate(sprite: &TextModeTextureAtlasSprite, rotation: u8) -> TextModeTextureAtlasSprite {
    let mut new_sprite = sprite.clone();
    new_sprite.rotation = (new_sprite.rotation + rotation) % 4;
    return new_sprite;
}

#[derive(Component)]
pub struct Wiggle(f32, usize);

impl Wiggle {
    pub fn with_frequency(f: f32) -> Self {
        Wiggle(f, 0)
    }

    pub fn slow() -> f32 { 0.05 }
}

pub fn wiggle(
    mut query: Query<(&mut Transform, &mut Wiggle)>,
) {
    for (mut pos, mut w) in query.iter_mut() {
        pos.translation.y += (w.0 * w.1 as f32).sin() * 0.75;
        pos.translation.x += (w.0 * w.1 as f32).cos() * 0.75;
        w.1 += 1;
    }
}