use bevy::app::App;
use bevy::prelude::*;

use crate::{GameState, util};
use crate::graphics::{sprite, text};
use crate::graphics::loading::{Fonts, Textures};
use crate::graphics::palette::Palette;
use crate::graphics::text::TextStyles;
use crate::util::size::f32_tile_to_f32;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                setup.in_schedule(OnEnter(GameState::Main))
            )
        ;
    }
}

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    textures: Res<Textures>,
) {
    for (x, y, text, style) in [
        (f32_tile_to_f32(1.), f32_tile_to_f32(1.25), "Level 1", TextStyles::Heading),
        (f32_tile_to_f32(1.), f32_tile_to_f32(0.5), "Haunted streets", TextStyles::Body),
    ] {
        commands
            .spawn(text::ttf(x, y, util::z_pos::GUI_FG,
                             text, style, &fonts, Palette::D))
        ;
    }

    for x in 0..util::size::WIDTH {
        for (i, y, bg, fg) in [
            (32, 2, Palette::E, Palette::D),
            (0, 1, Palette::E, Palette::Transparent),
            (0, 0, Palette::E, Palette::Transparent),
        ] {
            commands.spawn(sprite(
                i, x, y, util::z_pos::GUI_BG,
                bg, fg, false, 0,
                textures.tileset.clone(),
            ));
        }
    }
}