use bevy::app::App;
use bevy::prelude::*;

use crate::graphics::loading::Textures;
use crate::graphics::{MainBundle, Palette, sprite};
use crate::util::size;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_texts);
    }
}

#[derive(Component)]
pub struct Text {
    pub text: String,
    pub x: usize,
    pub y: usize,
    pub z: f32,
    bg: Palette,
    fg: Palette,
}

pub fn text(text: &str, x: usize, y: usize, z: f32) -> Text {
    Text {
        text: text.to_string(),
        x,
        y,
        z,
        bg: Palette::Black,
        fg: Palette::White,
    }
}

pub fn color_text(text: &str, x: usize, y: usize, z: f32, bg: Palette, fg: Palette) -> Text {
    Text {
        text: text.to_string(),
        x,
        y,
        z,
        bg,
        fg,
    }
}

pub fn from_middle(text: &str, x: isize, y: isize, z: f32, bg: Palette, fg: Palette) -> Text {
    color_text(
        text,
        (x - text.len() as isize / 2 + size::WIDTH as isize / 2) as usize,
        (y + size::HEIGHT as isize / 2) as usize,
        z, bg, fg,
    )
}

fn update_texts(
    mut commands: Commands,
    texts: Query<(&Text, Entity), Or<(Changed<Text>, Added<Text>)>>,
    textures: Option<Res<Textures>>,
) {
    let Some(textures) = textures else { return; };

    for (text, e) in texts.iter() {
        commands.entity(e).despawn_descendants();
        commands.entity(e).clear_children();
        commands.entity(e).insert(MainBundle::from_tiles(text.x, text.y, text.z));

        for (i, char) in text.text.chars().enumerate() {
            let child = commands.spawn(sprite(
                glyph_index(char).unwrap_or(0),
                i, 0, 0.,
                text.bg, text.fg,
                false, 0,
                textures.mrmotext.clone(),
            )).id();
            commands.entity(e).add_child(child);
        }
    }
}

pub fn glyph_index(c: char) -> Option<usize> {
    match c {
        '•' => Some(478),
        '°' => Some(439),
        '↔' => Some(79),
        '→' => Some(15),
        '←' => Some(47),
        'a'..='z' => Some(c as usize - 'a' as usize + 897),
        '!'..='_' => Some(c as usize - '!' as usize + 865),
        _ => None,
    }
}