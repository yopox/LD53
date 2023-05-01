use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::graphics::loading::Fonts;
use crate::graphics::Palette;

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
            TextStyles::Heading => 16.,
            TextStyles::Body => 16.,
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
    ttf_anchor(x, y, z, text, style, fonts, color, Anchor::BottomLeft)
}

pub fn ttf_anchor(x: f32, y: f32, z: f32, text: &str, style: TextStyles, fonts: &Fonts, color: Palette, text_anchor: Anchor) -> Text2dBundle {
    Text2dBundle {
        text: Text {
            sections: vec![
                TextSection::new(text, style.style(fonts, color)),
            ],
            ..default()
        },
        text_anchor,
        transform: Transform::from_xyz(x, y, z),
        ..default()
    }
}