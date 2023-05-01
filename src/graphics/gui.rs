use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::TextModeTextureAtlasSprite;
use strum::IntoEnumIterator;

use crate::{GameState, tower, util};
use crate::battle::{BattleUI, CursorState, Money};
use crate::collision::body_size;
use crate::graphics::{grid, MainBundle, sprite, sprite_f32, sprite_from_tile_with_alpha, text};
use crate::graphics::grid::Grid;
use crate::graphics::loading::{Fonts, Textures};
use crate::graphics::palette::Palette;
use crate::graphics::text::TextStyles;
use crate::tower::Towers;
use crate::util::size::{f32_tile_to_f32, is_oob, tile_to_f32};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Main)))
            .add_systems((update_money, update_cursor, update_popup, update_tower_button, place_tower).in_set(OnUpdate(GameState::Main)))
        ;
    }
}

#[derive(Component)]
struct Cursor;

#[derive(Resource)]
struct HoveredPos(pub (usize, usize));

#[derive(Component)]
struct TowerButton(Towers);

#[derive(Component)]
struct MoneyText;

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    textures: Res<Textures>,
) {
    // Text
    let left_margin = f32_tile_to_f32(2.);
    for (x, y, text, style) in [
        (left_margin, f32_tile_to_f32(3.6), "Level 1", TextStyles::Heading),
        (left_margin, f32_tile_to_f32(2.1), "Haunted streets", TextStyles::Body),
    ] {
        commands
            .spawn(text::ttf(
                x, y, util::z_pos::GUI_FG,
                text, style, &fonts, Palette::D,
            ))
        ;
    }

    commands
        .spawn(text::ttf(
            left_margin, f32_tile_to_f32(0.5), util::z_pos::GUI_FG,
            "€0", TextStyles::Heading, &fonts, Palette::D,
        ))
        .insert(MoneyText)
    ;

    // Background
    for x in 0..util::size::WIDTH {
        for y in 0..util::size::GUI_HEIGHT {
            let (i, bg, fg) = match y {
                5 => (418, Palette::E, Palette::D),
                _ => (416, Palette::E, Palette::Transparent),
            };
            commands.spawn(sprite(
                i, x, y, util::z_pos::GUI_BG,
                bg, fg, false, 0,
                textures.tileset.clone(),
            ));
        }
    }

    // Cursor
    commands
        .spawn(MainBundle::from_xyz(0., 0., util::z_pos::CURSOR))
        .with_children(|builder| {
            for (x, y, r) in [(0, 1, 0), (1, 1, 1), (1, 0, 2), (0, 0, 3)] {
                builder.spawn(sprite(
                    417, x, y, 0.,
                    Palette::Transparent, Palette::B,
                    false, r, textures.tileset.clone(),
                ));
            }
        })
        .insert(Cursor)
    ;

    // Tower buttons
    for (i, tower) in Towers::iter().enumerate() {
        commands
            .spawn(TowerButton(tower))
            .insert(MainBundle::from_xyz(tile_to_f32(14 + 4 * i), f32_tile_to_f32(2.), util::z_pos::GUI_FG))
            .with_children(|builder| {
                sprite_from_tile_with_alpha(builder, tower.get_tiles(), &textures.tileset, 0., ButtonState::CanBuild.get_alpha());
                builder.spawn(text::ttf_anchor(
                    f32_tile_to_f32(1.0), f32_tile_to_f32(0.3), util::z_pos::GUI_FG,
                    &format!("€{}", tower.get_cost()),
                    TextStyles::Heading, &fonts, Palette::D,
                    Anchor::TopCenter,
                ));
            });
    }
}

fn update_money(
    money: Res<Money>,
    mut text: Query<&mut Text, With<MoneyText>>,
) {
    if money.is_changed() || money.is_added() {
        let mut text = text.single_mut();
        text.sections[0].value = format!("€{}", money.0);
    }
}

fn update_cursor(
    mut commands: Commands,
    grid: Option<Res<Grid>>,
    windows: Query<&Window>,
    mut cursor: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
    hovered: Option<ResMut<HoveredPos>>,
) {
    let mut clean = || commands.remove_resource::<HoveredPos>();
    let Ok((mut pos, mut vis)) = cursor.get_single_mut() else {
        clean();
        return;
    };
    vis.set_if_neq(Visibility::Hidden);

    let Some(grid) = grid else {
        clean();
        return;
    };
    let grid = &grid.0;
    let Some(cursor_pos) = util::cursor_pos(windows) else {
        clean();
        return;
    };

    // Get hovered tile
    let tile_size = tile_to_f32(1);
    let (x, y) = (cursor_pos.x / tile_size, cursor_pos.y / tile_size);
    let (x, y) = (x as isize, y as isize - util::size::GUI_HEIGHT as isize);
    let (x, y) = (x / 2, y / 2);

    /// Set visibility and position on [grid::RoadElement::Rock] hover.
    if is_oob(x, y) { return; }

    if grid[y as usize][x as usize] == grid::RoadElement::Rock {
        vis.set_if_neq(Visibility::Inherited);
        let new_hovered = (x as usize, y as usize);
        match hovered {
            Some(mut res) if res.0.0 != new_hovered.0 || res.0.1 != new_hovered.1 => res.0 = new_hovered,
            Some(_) => {},
            None => commands.insert_resource(HoveredPos(new_hovered)),
        }
        pos.translation.x = tile_to_f32(x as usize * 2);
        pos.translation.y = tile_to_f32(y as usize * 2 + util::size::GUI_HEIGHT);
    } else {
        clean();
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

/// A 12*6 information popup showed on [Information] hover
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
        .insert(BattleUI)
        .with_children(|builder| {
            for y in 0..6 {
                for x in 0..12 {
                    let (i, r) = match (x, y) {
                        (0, 0) => (420, 3),
                        (11, 0) => (420, 2),
                        (11, 5) => (420, 1),
                        (0, 5) => (420, 0),
                        _ => (421, 0)
                    };
                    let mut bundle = sprite(
                        i, x, y, 0.,
                        Palette::Transparent, Palette::F,
                        false, r,
                        textures.tileset.clone(),
                    );
                    bundle.sprite.alpha = 0.75;
                    builder.spawn(bundle);
                }
            }

            let fg_z = util::z_pos::POPUP_FG - util::z_pos::POPUP_BG;

            for (text, style, y) in [(&info.name, TextStyles::Heading, 3.9), (&info.description, TextStyles::Body, 2.6)] {
                builder.spawn(text::ttf(
                    f32_tile_to_f32(1.), f32_tile_to_f32(y), fg_z,
                    text, style,
                    &fonts, Palette::B,
                ));
            }

            for (attr, y) in [(&info.attr1, 1.3), (&info.attr2, 0.3)] {
                if let Some((t, i)) = attr {
                    builder.spawn(text::ttf(
                        f32_tile_to_f32(1.), f32_tile_to_f32(y), fg_z,
                        t, TextStyles::Body,
                        &fonts, Palette::B,
                    ));

                    for x in 0..10 {
                        builder.spawn(sprite_f32(
                            419, f32_tile_to_f32(x as f32 / 2. + 6.), f32_tile_to_f32(y + 7. / 16.), fg_z,
                            Palette::Transparent, if x < *i { Palette::K } else { Palette::P },
                            false, 0, textures.tileset.clone(),
                        ));
                    }
                }
            }
        });
}

enum ButtonState {
    CantBuild,
    CanBuild,
    Selected,
}

impl ButtonState {
    fn get_alpha(&self) -> f32 {
        match self {
            ButtonState::CantBuild => 0.1,
            ButtonState::CanBuild => 0.5,
            ButtonState::Selected => 1.,
        }
    }
}

fn update_tower_button(
    cursor_state: Option<ResMut<CursorState>>,
    buttons: Query<(&TowerButton, &Transform, Entity)>,
    children: Query<&Children>,
    mut sprites: Query<&mut TextModeTextureAtlasSprite>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
    money: Res<Money>,
) {
    let Some(mut cursor_state) = cursor_state else { return; };
    let mut cursor_state = cursor_state;
    let Some(cursor_pos) = util::cursor_pos(windows) else { return; };
    let clicked = mouse.just_pressed(MouseButton::Left);

    for (button, pos, id) in &buttons {
        let mut button_state = ButtonState::CanBuild;
        if money.0 < button.0.get_cost() { button_state = ButtonState::CantBuild; } else if let CursorState::Build(t) = cursor_state.as_ref() {
            button_state = if *t == button.0 { ButtonState::Selected } else { ButtonState::CanBuild };
        } else {
            // Check button hover
            let size = body_size(button.0.get_tiles());
            let (x, y) = (pos.translation.x, pos.translation.y);
            let hover = cursor_pos.x >= x && cursor_pos.x <= x + size.x && cursor_pos.y >= y && cursor_pos.y <= y + size.y;
            button_state = if hover { ButtonState::Selected } else { ButtonState::CanBuild };
            if hover && clicked { cursor_state.set_if_neq(CursorState::Build(button.0)); }
        }

        for id in children.iter_descendants(id) {
            let Ok(mut sprite) = sprites.get_mut(id) else { continue };
            sprite.alpha = button_state.get_alpha();
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
    mut money: ResMut<Money>,
) {
    let Some(mut state) = state else { return; };
    let cursor_changed = match cursor {
        Some(ref res) => res.is_changed(),
        _ => false,
    };
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
                if cursor_changed {
                    // Update its position
                    let tower_pos = util::grid_to_tower_pos(x, y, *t);
                    pos.translation.x = tower_pos.x;
                    pos.translation.y = tower_pos.y;
                }

                if mouse.just_pressed(MouseButton::Left) {
                    // Build the tower
                    if money.0 >= t.get_cost() {
                        money.0 -= t.get_cost();
                        tower::place_tower(x, y, &mut commands, *t, &textures.tileset, &time);
                    }
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
            let tower_pos = util::grid_to_tower_pos(x, y, *t);
            commands
                .spawn(TransparentTower)
                .insert(MainBundle::from_xyz(tower_pos.x, tower_pos.y, util::z_pos::TOWERS))
                .with_children(|builder| {
                    sprite_from_tile_with_alpha(builder, t.get_tiles(), &textures.tileset, 0., 0.85);
                });
        }
    }
}