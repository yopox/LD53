use bevy::app::App;
use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;

use crate::{GameState, tower, util};
use crate::collision::body_size;
use crate::graphics::{grid, MainBundle, sprite, sprite_f32, sprite_from_tile_with_alpha, text};
use crate::graphics::grid::Grid;
use crate::graphics::loading::{Fonts, Textures};
use crate::graphics::palette::Palette;
use crate::graphics::text::TextStyles;
use crate::playing::{CursorState, PlayingUI};
use crate::tower::Towers;
use crate::util::size::{f32_tile_to_f32, is_oob, tile_to_f32};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Main)))
            .add_systems((update_cursor, update_popup, update_tower_button, place_tower).in_set(OnUpdate(GameState::Main)))
        ;
    }
}

#[derive(Component)]
struct Cursor;

#[derive(Resource)]
struct HoveredPos(pub (usize, usize));

#[derive(Component)]
struct TowerButton(Towers);

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
        .insert(Cursor)
    ;

    // Tower buttons
    commands
        .spawn(TowerButton(Towers::Basic))
        .insert(MainBundle::from_xyz(tile_to_f32(8), f32_tile_to_f32(0.5), util::z_pos::GUI_FG))
        .with_children(|builder| {
            sprite_from_tile_with_alpha(builder, Towers::Basic.get_tiles(), &textures.tileset, 0., util::misc::TRANSPARENT_TOWER_ALPHA);
        });
}

fn update_cursor(
    mut commands: Commands,
    grid: Option<Res<Grid>>,
    windows: Query<&Window>,
    mut cursor: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
) {
    commands.remove_resource::<HoveredPos>();
    let Ok((mut pos, mut vis)) = cursor.get_single_mut() else { return; };
    vis.set_if_neq(Visibility::Hidden);

    let Some(grid) = grid else { return; };
    let grid = &grid.0;
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };

    // Get hovered tile
    let tile_size = tile_to_f32(1);
    let (x, y) = (cursor_pos.x / tile_size, cursor_pos.y / tile_size);
    let (x, y) = (x as isize, y as isize - 3);

    /// Set visibility and position on [grid::RoadElement::Rock] hover.
    if is_oob(x, y) { return; }

    if grid[y as usize][x as usize] == grid::RoadElement::Rock {
        vis.set_if_neq(Visibility::Inherited);
        commands.insert_resource(HoveredPos((x as usize, y as usize)));
        pos.translation.x = tile_to_f32(x as usize);
        pos.translation.y = tile_to_f32(y as usize + util::size::GUI_HEIGHT);
    }
}

/// Add this component to entities that should display a popup on hover.
#[derive(Component)]
pub struct HoverPopup {
    name: String,
    description: String,
    attr1: Option<(String, u8)>,
    attr2: Option<(String, u8)>,
    width: f32,
    height: f32,
}

impl HoverPopup {
    pub fn new(name: &str, description: &str, attr1: Option<(&str, u8)>, attr2: Option<(&str, u8)>, width: f32, height: f32) -> Self {
        let unwrap = |opt: Option<(&str, u8)>| match opt {
            Some((t, i)) => Some((t.to_string(), i)),
            None => None,
        };

        Self {
            name: name.to_string(),
            description: description.to_string(),
            attr1: unwrap(attr1),
            attr2: unwrap(attr2),
            width,
            height,
        }
    }
}

/// A 6*3 information popup showed on [Information] hover
#[derive(Component)]
struct Popup(Entity);

fn update_popup(
    mut commands: Commands,
    hover_popup: Query<(&Transform, &HoverPopup, Entity), Without<Popup>>,
    popup: Query<(&Popup, Entity), Without<HoverPopup>>,
    windows: Query<&Window>,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
) {
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };

    let mut delete_popup = true;
    for (pos, info, id) in hover_popup.iter() {
        let (x, y) = (pos.translation.x, pos.translation.y);
        let (w, h) = (info.width, info.height);
        if cursor_pos.x >= x && cursor_pos.x <= x + w && cursor_pos.y >= y && cursor_pos.y <= y + h {
            let mut recreate_popup = false;
            delete_popup = false;

            match popup.get_single() {
                Ok((p, _)) if p.0 == id => {
                    // Popup exists -> correct entity -> do nothing
                }
                Ok((_, popup_id)) => {
                    // Popup exists -> wrong entity -> respawn popup
                    commands.entity(popup_id).despawn_recursive();
                    recreate_popup = true;
                }
                _ => {
                    // Popup doesn't exist -> spawn a popup
                    recreate_popup = true;
                }
            }

            if recreate_popup { spawn_popup(&mut commands, pos, id, info, &textures, &fonts); }
        }
    }

    if delete_popup {
        if let Ok((_, id)) = popup.get_single() { commands.entity(id).despawn_recursive(); }
    }
}

fn spawn_popup(
    commands: &mut Commands,
    owner_pos: &Transform,
    owner_id: Entity,
    info: &HoverPopup,
    textures: &Res<Textures>,
    fonts: &Res<Fonts>,
) {
    commands
        .spawn(MainBundle::from_xyz(
            owner_pos.translation.x + info.width + f32_tile_to_f32(0.5),
            owner_pos.translation.y + info.height - tile_to_f32(3),
            util::z_pos::POPUP_BG,
        ))
        .insert(Popup(owner_id))
        .insert(PlayingUI)
        .with_children(|builder| {
            for (i, x, y, r) in [
                (34, 0, 2, 0), (35, 1, 2, 0), (35, 2, 2, 0), (35, 3, 2, 0), (35, 4, 2, 0), (34, 5, 2, 1),
                (35, 0, 1, 0), (35, 1, 1, 0), (35, 2, 1, 0), (35, 3, 1, 0), (35, 4, 1, 0), (35, 5, 1, 0),
                (34, 0, 0, 3), (35, 1, 0, 0), (35, 2, 0, 0), (35, 3, 0, 0), (35, 4, 0, 0), (34, 5, 0, 2),
            ] {
                let mut bundle = sprite(
                    i, x, y, 0.,
                    Palette::Transparent, Palette::F,
                    false, r,
                    textures.tileset.clone(),
                );
                bundle.sprite.alpha = 0.75;
                builder.spawn(bundle);
            }

            let fg_z = util::z_pos::POPUP_FG - util::z_pos::POPUP_BG;

            for (text, style, y) in [(&info.name, TextStyles::Heading, 1.95), (&info.description, TextStyles::Body, 1.3)] {
                builder.spawn(text::ttf(
                    f32_tile_to_f32(0.5), f32_tile_to_f32(y), fg_z,
                    text, style,
                    &fonts, Palette::B,
                ));
            }

            for (attr, y) in [(&info.attr1, 0.65), (&info.attr2, 0.15)] {
                if let Some((t, i)) = attr {
                    builder.spawn(text::ttf(
                        f32_tile_to_f32(0.5), f32_tile_to_f32(y), fg_z,
                        t, TextStyles::Body,
                        &fonts, Palette::B,
                    ));

                    for x in 0..10 {
                        builder.spawn(sprite_f32(
                            36, f32_tile_to_f32(x as f32 / 4. + 3.), f32_tile_to_f32(y + 7. / 32.), fg_z,
                            Palette::Transparent, if x < *i { Palette::K } else { Palette::P },
                            false, 0, textures.tileset.clone(),
                        ));
                    }
                }
            }
        });
}

fn update_tower_button(
    cursor_state: Option<ResMut<CursorState>>,
    buttons: Query<(&TowerButton, &Transform, Entity)>,
    children: Query<&Children>,
    mut sprites: Query<&mut TextModeTextureAtlasSprite>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
) {
    let Some(mut cursor_state) = cursor_state else { return; };
    let mut cursor_state = cursor_state;
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };
    let clicked = mouse.just_pressed(MouseButton::Left);

    for (button, pos, id) in &buttons {
        let mut transparent = true;
        if let CursorState::Build(t) = cursor_state.as_ref() {
            transparent = *t != button.0;
        } else {
            // Check button hover
            let size = body_size(button.0.get_tiles());
            let (x, y) = (pos.translation.x, pos.translation.y);
            let hover = cursor_pos.x >= x && cursor_pos.x <= x + size.x && cursor_pos.y >= y && cursor_pos.y <= y + size.y;
            transparent = !hover;
            if hover && clicked { cursor_state.set_if_neq(CursorState::Build(button.0)); }
        }

        for id in children.iter_descendants(id) {
            let Ok(mut sprite) = sprites.get_mut(id) else { continue };
            sprite.alpha = if transparent { util::misc::TRANSPARENT_TOWER_ALPHA } else { 1.0 };
        }
    }
}

#[derive(Component)]
struct TransparentTower;

fn place_tower(
    mut commands: Commands,
    mut state: Option<ResMut<CursorState>>,
    cursor: Option<Res<HoveredPos>>,
    mut transparent_tower: Query<(&mut Transform, Entity), With<TransparentTower>>,
    textures: Res<Textures>,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Some(mut state) = state else { return; };
    let cursor: Option<(usize, usize)> = match cursor {
        Some(_) => Some(cursor.unwrap().0),
        _ => None,
    };

    // Escape [CursorState::Build]
    if mouse.just_pressed(MouseButton::Right) || keys.just_pressed(KeyCode::Escape) {
        state.set_if_neq(CursorState::Select);
    }
    let state = state.as_ref();

    if let Ok((mut pos, id)) = transparent_tower.get_single_mut() {
        // The transparent tower exists
        match (state, cursor) {
            (CursorState::Build(t), Some((x, y))) => {
                // Update its position
                pos.translation.x = tile_to_f32(x);
                pos.translation.y = tile_to_f32(y + util::size::GUI_HEIGHT);

                if mouse.just_pressed(MouseButton::Left) {
                    // Build the tower
                    // TODO: Check money
                    tower::place_tower(x, y, &mut commands, *t, &textures.tileset, &time);
                    commands.insert_resource(CursorState::Select);
                    return;
                }
            }
            _ => {
                // Delete the tower :(
                commands.entity(id).despawn_recursive();
            }
        }
    } else {
        // There is no transparent tower
        if let CursorState::Build(t) = state {
            let Some((x, y)) = cursor else { return; };
            commands
                .spawn(TransparentTower)
                .insert(MainBundle::from_xyz(tile_to_f32(x), tile_to_f32(y + util::size::GUI_HEIGHT), util::z_pos::TOWERS))
                .with_children(|builder| {
                    sprite_from_tile_with_alpha(builder, t.get_tiles(), &textures.tileset, 0., 0.85);
                });
        }
    }
}