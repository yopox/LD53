use bevy::app::App;
use bevy::prelude::*;

use crate::{GameState, util};
use crate::graphics::loading::Fonts;
use crate::graphics::palette::Palette;
use crate::graphics::text;
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
) {
    for (x, y, text, style) in [
        (f32_tile_to_f32(1.), f32_tile_to_f32(1.0), "Level 1", TextStyles::Heading),
        (f32_tile_to_f32(1.), f32_tile_to_f32(0.25), "Haunted streets", TextStyles::Body),
    ] {
        commands
            .spawn(text::ttf(x, y, util::z_pos::GUI,
                             text, style, &fonts, Palette::D))
        ;
    }
}