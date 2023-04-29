use bevy::prelude::Color;
use lazy_static::lazy_static;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Palette {
    Black = 0,
    White,
    Transparent,
}

impl Into<Color> for Palette {
    fn into(self) -> Color {
        COLOR_OF_PALETTE[self as usize]
    }
}

lazy_static! {
    static ref COLOR_OF_PALETTE: [Color; 3] = [
        Color::hex("#000000").unwrap(),
        Color::hex("#FFFFFF").unwrap(),
        Color::hex("#00000000").unwrap(),
    ];
}