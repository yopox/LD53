use bevy::app::App;
use bevy::prelude::*;

use crate::{GameState, util};
use crate::graphics::{grid, sprite, text};
use crate::graphics::grid::Grid;
use crate::graphics::loading::{Fonts, Textures};
use crate::graphics::palette::Palette;
use crate::graphics::text::TextStyles;
use crate::util::size::{f32_tile_to_f32, is_oob, tile_to_f32};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Main)))
            .add_system(update_cursor.in_set(OnUpdate(GameState::Main)))
        ;
    }
}

#[derive(Component)]
struct Cursor {
    hover_pos: Option<(usize, usize)>,
}

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    textures: Res<Textures>,
) {
    // Text
    for (x, y, text, style) in [
        (f32_tile_to_f32(1.), f32_tile_to_f32(1.25), "Level 1", TextStyles::Heading),
        (f32_tile_to_f32(1.), f32_tile_to_f32(0.5), "Haunted streets", TextStyles::Body),
    ] {
        commands
            .spawn(text::ttf(x, y, util::z_pos::GUI_FG,
                             text, style, &fonts, Palette::D))
        ;
    }

    // Background
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

    // Cursor
    commands
        .spawn(sprite(
            33, 4, 4, util::z_pos::CURSOR,
            Palette::Transparent, Palette::B,
            false, 0, textures.tileset.clone(),
        ))
        .insert(Cursor { hover_pos: None })
    ;
}

fn update_cursor(
    grid: Option<Res<Grid>>,
    mut windows: Query<&mut Window>,
    mut cursor: Query<(&mut Transform, &mut Visibility, &mut Cursor)>,
) {
    let Ok((mut pos,
               mut vis,
               mut cursor)) = cursor.get_single_mut() else { return; };
    vis.set_if_neq(Visibility::Hidden);
    cursor.hover_pos = None;

    let Some(grid) = grid else { return; };
    let grid = &grid.0;
    let window = windows.get_single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else { return; };

    // Get hovered tile
    let tile_size = util::size::SCALE * tile_to_f32(1);
    let (x, y) = (cursor_pos.x / tile_size, cursor_pos.y / tile_size);
    let (x, y) = (x as isize, y as isize - 3);

    /// Set visibility and position on [grid::RoadElement::Rock] hover.
    if is_oob(x, y) { return; }

    if grid[y as usize][x as usize] == grid::RoadElement::Rock {
        vis.set_if_neq(Visibility::Inherited);
        cursor.hover_pos = Some((x as usize, y as usize));
        pos.translation.x = tile_to_f32(x as usize);
        pos.translation.y = tile_to_f32(y as usize + util::size::GUI_HEIGHT);
    }
}