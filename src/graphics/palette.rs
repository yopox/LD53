use bevy::prelude::Color;
use lazy_static::lazy_static;

#[repr(u8)]
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Palette {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Transparent,
}

impl Into<Color> for Palette {
    fn into(self) -> Color {
        COLOR_OF_PALETTE[self as usize]
    }
}

impl Into<Palette> for u8 {
    fn into(self) -> Palette {
        match self {
            0 => Palette::A,
            1 => Palette::B,
            2 => Palette::C,
            3 => Palette::D,
            4 => Palette::E,
            5 => Palette::F,
            6 => Palette::G,
            7 => Palette::H,
            8 => Palette::I,
            9 => Palette::J,
            10 => Palette::K,
            11 => Palette::L,
            12 => Palette::M,
            13 => Palette::N,
            14 => Palette::O,
            15 => Palette::P,
            _ => Palette::Transparent,
        }
    }
}

lazy_static! {
    static ref COLOR_OF_PALETTE: [Color; 17] = [
        Color::hex("#ffffff").unwrap(),
        Color::hex("#6df7c1").unwrap(),
        Color::hex("#11adc1").unwrap(),
        Color::hex("#606c81").unwrap(),
        Color::hex("#393457").unwrap(),
        Color::hex("#1e8875").unwrap(),
        Color::hex("#5bb361").unwrap(),
        Color::hex("#a1e55a").unwrap(),
        Color::hex("#f7e476").unwrap(),
        Color::hex("#f99252").unwrap(),
        Color::hex("#cb4d68").unwrap(),
        Color::hex("#6a3771").unwrap(),
        Color::hex("#c92464").unwrap(),
        Color::hex("#f48cb6").unwrap(),
        Color::hex("#f7b69e").unwrap(),
        Color::hex("#9b9c82").unwrap(),
        Color::hex("#00000000").unwrap(),
    ];
}