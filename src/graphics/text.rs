use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::graphics::{MainBundle, Palette, sprite};
use crate::graphics::loading::{Fonts, Textures};
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
        bg: Palette::E,
        fg: Palette::A,
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
                textures.tileset.clone(),
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

pub enum TextStyles {
    Heading,
    Body,
}

impl TextStyles {
    fn get_font(&self, fonts: &Fonts) -> Handle<Font> {
        match self {
            TextStyles::Heading => fonts.yesterday.clone(),
            TextStyles::Body => fonts.axones.clone(),
        }
    }

    fn get_size(&self) -> f32 {
        match self {
            TextStyles::Heading => 8.,
            TextStyles::Body => 8.,
        }
    }

    pub fn style(&self, fonts: &Fonts, color: Palette) -> TextStyle {
        TextStyle {
            font: self.get_font(fonts),
            font_size: self.get_size(),
            color: color.into(),
        }
    }
}

pub fn ttf(x: f32, y: f32, z: f32, text: &str, style: TextStyles, fonts: &Fonts, color: Palette) -> Text2dBundle {
    Text2dBundle {
        text: bevy::text::Text {
            sections: vec![
                TextSection::new(text, style.style(fonts, color)),
            ],
            ..default()
        },
        text_anchor: Anchor::BottomLeft,
        transform: Transform::from_xyz(x, y, z),
        ..default()
    }
}